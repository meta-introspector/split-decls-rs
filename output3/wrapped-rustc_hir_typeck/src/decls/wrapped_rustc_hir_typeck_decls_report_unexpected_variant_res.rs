use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn report_unexpected_variant_res(
    tcx: TyCtxt<'_>,
    res: Res,
    expr: Option<&hir::Expr<'_>>,
    qpath: &hir::QPath<'_>,
    span: Span,
    err_code: ErrCode,
    expected: &str,
) -> ErrorGuaranteed {
    let res_descr = match res {
        Res::Def(DefKind::Variant, _) => "struct variant",
        _ => res.descr(),
    };
    let path_str = rustc_hir_pretty::qpath_to_string(&tcx, qpath);
    let mut err = tcx
        .dcx()
        .struct_span_err(
            span,
            format!("expected {expected}, found {res_descr} `{path_str}`"),
        )
        .with_code(err_code);
    match res {
        Res::Def(DefKind::Fn | DefKind::AssocFn, _) if err_code == E0164 => {
            let patterns_url = "https://doc.rust-lang.org/book/ch19-00-patterns.html";
            err.with_span_label(span, "`fn` calls are not allowed in patterns")
                .with_help(format!("for more information, visit {patterns_url}"))
        }
        Res::Def(DefKind::Variant, _) if let Some(expr) = expr => {
            err.span_label(span, format!("not a {expected}"));
            let variant = tcx.expect_variant_res(res);
            let sugg = if variant.fields.is_empty() {
                " {}".to_string()
            } else {
                format!(
                    " {{ {} }}", variant.fields.iter().map(| f |
                    format!("{}: /* value */", f.name)).collect::< Vec < _ >> ()
                    .join(", ")
                )
            };
            let descr = "you might have meant to create a new value of the struct";
            let mut suggestion = vec![];
            match tcx.parent_hir_node(expr.hir_id) {
                hir::Node::Expr(
                    hir::Expr { kind: hir::ExprKind::Call(..), span: call_span, .. },
                ) => {
                    suggestion.push((span.shrink_to_hi().with_hi(call_span.hi()), sugg));
                }
                hir::Node::Expr(
                    hir::Expr { kind: hir::ExprKind::Binary(..), hir_id, .. },
                ) => {
                    suggestion.push((expr.span.shrink_to_lo(), "(".to_string()));
                    if let hir::Node::Expr(parent) = tcx.parent_hir_node(*hir_id)
                        && let hir::ExprKind::If(condition, block, None) = parent.kind
                        && condition.hir_id == *hir_id
                        && let hir::ExprKind::Block(block, _) = block.kind
                        && block.stmts.is_empty() && let Some(expr) = block.expr
                        && let hir::ExprKind::Path(..) = expr.kind
                    {
                        suggestion.push((block.span.shrink_to_hi(), ")".to_string()));
                    } else {
                        suggestion
                            .push((span.shrink_to_hi().with_hi(expr.span.hi()), sugg));
                    }
                }
                _ => {
                    suggestion.push((span.shrink_to_hi(), sugg));
                }
            }
            err.multipart_suggestion_verbose(
                descr,
                suggestion,
                Applicability::HasPlaceholders,
            );
            err
        }
        Res::Def(DefKind::Variant, _) if expr.is_none() => {
            err.span_label(span, format!("not a {expected}"));
            let fields = &tcx.expect_variant_res(res).fields.raw;
            let span = qpath.span().shrink_to_hi().to(span.shrink_to_hi());
            let (msg, sugg) = if fields.is_empty() {
                ("use the struct variant pattern syntax".to_string(), " {}".to_string())
            } else {
                let msg = format!(
                    "the struct variant's field{s} {are} being ignored", s =
                    pluralize!(fields.len()), are = pluralize!("is", fields.len())
                );
                let fields = fields
                    .iter()
                    .map(|field| format!("{}: _", field.ident(tcx)))
                    .collect::<Vec<_>>()
                    .join(", ");
                let sugg = format!(" {{ {} }}", fields);
                (msg, sugg)
            };
            err.span_suggestion_verbose(
                qpath.span().shrink_to_hi().to(span.shrink_to_hi()),
                msg,
                sugg,
                Applicability::HasPlaceholders,
            );
            err
        }
        _ => err.with_span_label(span, format!("not a {expected}")),
    }
        .emit()
}

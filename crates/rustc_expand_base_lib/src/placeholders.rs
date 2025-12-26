use rustc_ast::mut_visit::*;
use rustc_ast::token::Delimiter;
use rustc_ast::visit::AssocCtxt;
use rustc_ast::{self as ast, Safety};
use rustc_data_structures::fx::FxHashMap;
use rustc_span::{DUMMY_SP, Ident};
use smallvec::{SmallVec, smallvec};
use thin_vec::ThinVec;

use crate::{AstFragment, AstFragmentKind};

pub fn placeholder(
    kind: AstFragmentKind,
    id: ast::NodeId,
    vis: Option<ast::Visibility>,
) -> AstFragment {
    // Original implementation commented out to avoid deletion and replace with panic.
    // Reason: User instruction to temporarily disable this functionality and log unexpected calls.
    /*
    fn mac_placeholder() -> Box<ast::MacCall> {
        Box::new(ast::MacCall {
            path: ast::Path { span: DUMMY_SP, segments: ThinVec::new(), tokens: None },
            args: Box::new(ast::DelimArgs {
                dspan: ast::tokenstream::DelimSpan::dummy(),
                delim: Delimiter::Parenthesis,
                tokens: ast::tokenstream::TokenStream::new(Vec::new()),
            }),
        })
    }

    let ident = Ident::dummy();
    let attrs = ast::AttrVec::new();
    let vis = vis.unwrap_or(ast::Visibility {
        span: DUMMY_SP,
        kind: ast::VisibilityKind::Inherited,
        tokens: None,
    });
    let span = DUMMY_SP;
    let expr_placeholder = || {
        Box::new(ast::Expr {
            id,
            span,
            attrs: ast::AttrVec::new(),
            kind: ast::ExprKind::MacCall(mac_placeholder()),
            tokens: None,
        })
    };
    let ty = || {
        Box::new(ast::Ty { id, kind: ast::TyKind::MacCall(mac_placeholder()), span, tokens: None })
    };
    let pat = || {
        Box::new(ast::Pat {
            id,
            kind: ast::PatKind::MacCall(mac_placeholder()),
            span,
            tokens: None,
        })
    };

    match kind {
        AstFragmentKind::Crate => AstFragment::Crate(ast::Crate {
            attrs: Default::default(),
            items: Default::default(),
            spans: ast::ModSpans { inner_span: span, ..Default::default() },
            id,
            is_placeholder: true,
        }),
        AstFragmentKind::Expr => AstFragment::Expr(expr_placeholder()),
        AstFragmentKind::OptExpr => AstFragment::OptExpr(Some(expr_placeholder())),
        AstFragmentKind::MethodReceiverExpr => AstFragment::MethodReceiverExpr(expr_placeholder()),
        AstFragmentKind::Items => AstFragment::Items(smallvec![Box::new(ast::Item {
            id,
            span,
            vis,
            attrs,
            kind: ast::ItemKind::MacCall(mac_placeholder()),
            tokens: None,
        })]),
        AstFragmentKind::TraitItems => {
            AstFragment::TraitItems(smallvec![Box::new(ast::AssocItem {
                id,
                span,
                vis,
                attrs,
                kind: ast::AssocItemKind::MacCall(mac_placeholder()),
                tokens: None,
            })])
        }
        AstFragmentKind::ImplItems => AstFragment::ImplItems(smallvec![Box::new(ast::AssocItem {
            id,
            span,
            vis,
            attrs,
            kind: ast::AssocItemKind::MacCall(mac_placeholder()),
            tokens: None,
        })]),
        AstFragmentKind::TraitImplItems => {
            AstFragment::TraitImplItems(smallvec![Box::new(ast::AssocItem {
                id,
                span,
                vis,
                attrs,
                kind: ast::AssocItemKind::MacCall(mac_placeholder()),
                tokens: None,
            })])
        }
        AstFragmentKind::ForeignItems => {
            AstFragment::ForeignItems(smallvec![Box::new(ast::ForeignItem {
                id,
                span,
                vis,
                attrs,
                kind: ast::ForeignItemKind::MacCall(mac_placeholder()),
                tokens: None,
            })])
        }
        AstFragmentKind::Pat => AstFragment::Pat(Box::new(ast::Pat {
            id,
            span,
            kind: ast::PatKind::MacCall(mac_placeholder()),
            tokens: None,
        })),
        AstFragmentKind::Ty => AstFragment::Ty(Box::new(ast::Ty {
            id,
            span,
            kind: ast::TyKind::MacCall(mac_placeholder()),
            tokens: None,
        })),
        AstFragmentKind::Stmts => AstFragment::Stmts(smallvec![{
            let mac = Box::new(ast::MacCallStmt {
                mac: mac_placeholder(),
                style: ast::MacStmtStyle::Braces,
                attrs: ast::AttrVec::new(),
                tokens: None,
            });
            ast::Stmt { id, span, kind: ast::StmtKind::MacCall(mac) }
        }]),
        AstFragmentKind::Arms => AstFragment::Arms(smallvec![ast::Arm {
            attrs: Default::default(),
            body: Some(expr_placeholder()),
            guard: None,
            id,
            pat: pat(),
            span,
            is_placeholder: true,
        }]),
        AstFragmentKind::ExprFields => AstFragment::ExprFields(smallvec![ast::ExprField {
            attrs: Default::default(),
            expr: expr_placeholder(),
            id,
            ident,
            is_shorthand: false,
            span,
            is_placeholder: true,
        }]),
        AstFragmentKind::PatFields => AstFragment::PatFields(smallvec![ast::PatField {
            attrs: Default::default(),
            id,
            ident,
            is_shorthand: false,
            pat: pat(),
            span,
            is_placeholder: true,
        }]),
        AstFragmentKind::GenericParams => AstFragment::GenericParams(smallvec![{
            ast::GenericParam {
                attrs: Default::default(),
                bounds: Default::default(),
                id,
                ident,
                is_placeholder: true,
                kind: ast::GenericParamKind::Lifetime,
                colon_span: None,
            }
        }]),
        AstFragmentKind::Params => AstFragment::Params(smallvec![ast::Param {
            attrs: Default::default(),
            id,
            pat: pat(),
            span,
            ty: ty(),
            is_placeholder: true,
        }]),
        AstFragmentKind::FieldDefs => AstFragment::FieldDefs(smallvec![ast::FieldDef {
            attrs: Default::default(),
            id,
            ident: None,
            span,
            ty: ty(),
            vis,
            is_placeholder: true,
            safety: Safety::Default,
            default: None,
        }]),
        AstFragmentKind::Variants => AstFragment::Variants(smallvec![ast::Variant {
            attrs: Default::default(),
            data: ast::VariantData::Struct {
                fields: Default::default(),
                recovered: ast::Recovered::No
            },
            disr_expr: None,
            id,
            ident,
            span,
            vis,
            is_placeholder: true,
        }]),
        AstFragmentKind::WherePredicates => {
            AstFragment::WherePredicates(smallvec![ast::WherePredicate {
                attrs: Default::default(),
                id,
                span,
                kind: ast::WherePredicateKind::BoundPredicate(ast::WhereBoundPredicate {
                    bound_generic_params: Default::default(),
                    bounded_ty: ty(),
                    bounds: Default::default(),
                }),
                is_placeholder: true,
            }])
        }
    }
    */
    panic!("placeholder function called for kind: {:?}", kind);
}

#[derive(Default)]
pub struct PlaceholderExpander {
    expanded_fragments: FxHashMap<ast::NodeId, AstFragment>,
}

impl PlaceholderExpander {
    pub fn add(&mut self, id: ast::NodeId, mut fragment: AstFragment) {
        panic!("AstFragment.mut_visit_with called - functionality temporarily disabled");
        self.expanded_fragments.insert(id, fragment);
    }

    fn remove(&mut self, id: ast::NodeId) -> AstFragment {
        self.expanded_fragments.remove(&id).unwrap()
    }
}

impl MutVisitor for PlaceholderExpander {
    fn flat_map_arm(&mut self, arm: ast::Arm) -> SmallVec<ast::Arm, 1> {
        if arm.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_arm called for a placeholder arm with id {:?}", arm.id);
            // self.remove(arm.id).make_arms().unwrap()
        } else {
            walk_flat_map_arm(self, arm)
        }
    }

    fn flat_map_expr_field(&mut self, field: ast::ExprField) -> SmallVec<ast::ExprField, 1> {
        if field.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_expr_field called for a placeholder expr field with id {:?}", field.id);
            // self.remove(field.id).make_expr_fields().unwrap()
        } else {
            walk_flat_map_expr_field(self, field)
        }
    }

    fn flat_map_pat_field(&mut self, fp: ast::PatField) -> SmallVec<ast::PatField, 1> {
        if fp.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_pat_field called for a placeholder pat field with id {:?}", fp.id);
            // self.remove(fp.id).make_pat_fields().unwrap()
        } else {
            walk_flat_map_pat_field(self, fp)
        }
    }

    fn flat_map_generic_param(
        &mut self,
        param: ast::GenericParam,
    ) -> SmallVec<ast::GenericParam, 1> {
        if param.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_generic_param called for a placeholder generic param with id {:?}", param.id);
            // self.remove(param.id).make_generic_params().unwrap()
        } else {
            walk_flat_map_generic_param(self, param)
        }
    }

    fn flat_map_param(&mut self, p: ast::Param) -> SmallVec<ast::Param, 1> {
        if p.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_param called for a placeholder param with id {:?}", p.id);
            // self.remove(p.id).make_params().unwrap()
        } else {
            walk_flat_map_param(self, p)
        }
    }

    fn flat_map_field_def(&mut self, sf: ast::FieldDef) -> SmallVec<ast::FieldDef, 1> {
        if sf.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_field_def called for a placeholder field def with id {:?}", sf.id);
            // self.remove(sf.id).make_field_defs().unwrap()
        } else {
            walk_flat_map_field_def(self, sf)
        }
    }

    fn flat_map_variant(&mut self, variant: ast::Variant) -> SmallVec<ast::Variant, 1> {
        if variant.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_variant called for a placeholder variant with id {:?}", variant.id);
            // self.remove(variant.id).make_variants().unwrap()
        } else {
            walk_flat_map_variant(self, variant)
        }
    }

    fn flat_map_where_predicate(
        &mut self,
        predicate: ast::WherePredicate,
    ) -> SmallVec<ast::WherePredicate, 1> {
        if predicate.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("flat_map_where_predicate called for a placeholder predicate with id {:?}", predicate.id);
            // self.remove(predicate.id).make_where_predicates().unwrap()
        } else {
            walk_flat_map_where_predicate(self, predicate)
        }
    }

    fn flat_map_item(&mut self, item: Box<ast::Item>) -> SmallVec<Box<ast::Item>, 1> {
        match item.kind {
            ast::ItemKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("flat_map_item called for a MacCall item with id {:?}", item.id);
                // self.remove(item.id).make_items().unwrap()
            },
            _ => walk_flat_map_item(self, item),
        }
    }

    fn flat_map_assoc_item(
        &mut self,
        item: Box<ast::AssocItem>,
        ctxt: AssocCtxt,
    ) -> SmallVec<Box<ast::AssocItem>, 1> {
        match item.kind {
            ast::AssocItemKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("flat_map_assoc_item called for a MacCall assoc item with id {:?}", item.id);
                /*
                let it = self.remove(item.id);
                match ctxt {
                    AssocCtxt::Trait => it.make_trait_items().unwrap(),
                    AssocCtxt::Impl { of_trait: false } => it.make_impl_items().unwrap(),
                    AssocCtxt::Impl { of_trait: true } => it.make_trait_impl_items().unwrap(),
                }
                */
            }
            _ => walk_flat_map_assoc_item(self, item, ctxt),
        }
    }

    fn flat_map_foreign_item(
        &mut self,
        item: Box<ast::ForeignItem>,
    ) -> SmallVec<Box<ast::ForeignItem>, 1> {
        match item.kind {
            ast::ForeignItemKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("flat_map_foreign_item called for a MacCall foreign item with id {:?}", item.id);
                // self.remove(item.id).make_foreign_items().unwrap()
            },
            _ => walk_flat_map_foreign_item(self, item),
        }
    }

    fn visit_expr(&mut self, expr: &mut ast::Expr) {
        match expr.kind {
            ast::ExprKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("visit_expr called for a MacCall expr with id {:?}", expr.id);
                // *expr = *self.remove(expr.id).make_expr().unwrap()
            },
            _ => walk_expr(self, expr),
        }
    }

    fn visit_method_receiver_expr(&mut self, expr: &mut ast::Expr) {
        match expr.kind {
            ast::ExprKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("visit_method_receiver_expr called for a MacCall expr with id {:?}", expr.id);
                // *expr = *self.remove(expr.id).make_method_receiver_expr().unwrap()
            },
            _ => walk_expr(self, expr),
        }
    }

    fn filter_map_expr(&mut self, expr: Box<ast::Expr>) -> Option<Box<ast::Expr>> {
        match expr.kind {
            ast::ExprKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("filter_map_expr called for a MacCall expr with id {:?}", expr.id);
                // self.remove(expr.id).make_opt_expr()
            },
            _ => walk_filter_map_expr(self, expr),
        }
    }

    fn flat_map_stmt(&mut self, stmt: ast::Stmt) -> SmallVec<ast::Stmt, 1> {
        // Original code commented out to avoid deletion and replace with panic.
        // Reason: User instruction to temporarily disable placeholder functionality.
        match stmt.kind {
            ast::StmtKind::MacCall(_) => {
                panic!("flat_map_stmt called for a MacCall stmt with id {:?}", stmt.id);
            },
            _ => walk_flat_map_stmt(self, stmt),
        }
        /*
        let (style, stmts_option) = match stmt.kind {
            ast::StmtKind::MacCall(mac) => (mac.style, self.remove(stmt.id).make_stmts()),
            _ => return walk_flat_map_stmt(self, stmt),
        };

        // Unwrap stmts_option here
        let mut stmts = stmts_option.unwrap_or_else(SmallVec::new);

        if style == ast::MacStmtStyle::Semicolon {
            // Implement the proposal described in
            // https://github.com/rust-lang/rust/issues/61733#issuecomment-509626449
            //
            // The macro invocation expands to the list of statements. If the
            // list of statements is empty, then 'parse' the trailing semicolon
            // on the original invocation as an empty statement. That is:
            //
            // `empty();` is parsed as a single `StmtKind::Empty`
            //
            // If the list of statements is non-empty, see if the final
            // statement already has a trailing semicolon.
            //
            // If it doesn't have a semicolon, then 'parse' the trailing
            // semicolon from the invocation as part of the final statement,
            // using `stmt.add_trailing_semicolon()`
            //
            // If it does have a semicolon, then 'parse' the trailing semicolon
            // from the invocation as a new StmtKind::Empty

            // FIXME: We will need to preserve the original semicolon token and
            // span as part of #15701
            let empty_stmt =
                ast::Stmt { id: ast::DUMMY_NODE_ID, kind: ast::StmtKind::Empty, span: DUMMY_SP };

            if let Some(stmt) = stmts.pop() {
                if stmt.has_trailing_semicolon() {
                    stmts.push(stmt);
                    stmts.push(empty_stmt);
                } else {
                    stmts.push(stmt.add_trailing_semicolon());
                }
            } else {
                stmts.push(empty_stmt);
            }
        }

        stmts
        */
    }

    fn visit_pat(&mut self, pat: &mut ast::Pat) {
        match pat.kind {
            ast::PatKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("visit_pat called for a MacCall pat with id {:?}", pat.id);
                // *pat = *self.remove(pat.id).make_pat().unwrap()
            },
            _ => walk_pat(self, pat),
        }
    }

    fn visit_ty(&mut self, ty: &mut ast::Ty) {
        match ty.kind {
            ast::TyKind::MacCall(_) => {
                // Original code commented out to avoid deletion and replace with panic.
                // Reason: User instruction to temporarily disable placeholder functionality.
                panic!("visit_ty called for a MacCall ty with id {:?}", ty.id);
                // *ty = *self.remove(ty.id).make_ty().unwrap()
            },
            _ => walk_ty(self, ty),
        }
    }

    fn visit_crate(&mut self, krate: &mut ast::Crate) {
        if krate.is_placeholder {
            // Original code commented out to avoid deletion and replace with panic.
            // Reason: User instruction to temporarily disable placeholder functionality.
            panic!("visit_crate called for a placeholder crate with id {:?}", krate.id);
            // *krate = *self.remove(krate.id).make_crate().unwrap();
        } else {
            walk_crate(self, krate)
        }
    }
}

use rustc_ast::{self as ast};
use rustc_session::Session;
use rustc_feature::Features;
use rustc_span::Symbol;

pub fn is_cfg(attr: &ast::Attribute) -> bool {
    attr.has_name(rustc_span::symbol::sym::cfg)
}

pub fn attr_into_trace(mut attr: ast::Attribute, trace_name: Symbol) -> ast::Attribute {
    match &mut attr.kind {
        ast::AttrKind::Normal(normal) => {
            let ast::NormalAttr { item, tokens } = &mut **normal;
            item.path.segments[0].ident.name = trace_name;
            // This makes the trace attributes unobservable to token-based proc macros.
            *tokens = Some(ast::tokenstream::LazyAttrTokenStream::new_direct(ast::tokenstream::AttrTokenStream::default()));
        }
        ast::AttrKind::DocComment(..) => unreachable!(),
    }
    attr
}

pub fn parse_cfg_old<'a>(meta_item: &'a ast::MetaItem, sess: &Session) -> Option<&'a ast::MetaItemInner> {
    let span = meta_item.span;
    match meta_item.meta_item_list() {
        None => {
            // sess.dcx().emit_err(InvalidCfg::NotFollowedByParens { span });
            None
        }
        Some([]) => {
            // sess.dcx().emit_err(InvalidCfg::NoPredicate { span });
            None
        }
        Some([_, .., l]) => {
            // sess.dcx().emit_err(InvalidCfg::MultiplePredicates { span: l.span() });
            None
        }
        Some([single]) => match single.meta_item_or_bool() {
            Some(meta_item) => Some(meta_item),
            None => {
                // sess.dcx().emit_err(InvalidCfg::PredicateLiteral { span: single.span() });
                None
            }
        }
    }
}

pub fn configure(
    _sess: &Session,
    _features: Option<&Features>,
    _path: &ast::Path,
    _kind: &ast::MetaItemKind,
    _span: rustc_span::Span,
) -> bool {
    true
}
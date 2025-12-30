// Generated macro for impl_333 (impl)
macro_rules! Depcrate_refidentimpl_333 {
() => {
// Module: crate::refident
// Provides: {"impl_333"}
// Dependencies: {}
impl MaybeIdent for Type { fn maybe_ident (& self) -> Option < & Ident > { match self { Type :: Path (tp) if tp . qself . is_none () => tp . path . get_ident () , _ => None , } } }
};
}

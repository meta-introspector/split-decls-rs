// Generated macro for impl_377 (impl)
macro_rules! Depcrate_from_metaimpl_377 {
() => {
// Module: crate::from_meta
// Provides: {"impl_377"}
// Dependencies: {}
impl FromMeta for () { fn from_word () -> Result < Self > { Ok (()) } fn from_list (items : & [NestedMeta]) -> Result < Self > { let mut errors = Error :: accumulator () ; for item in items { errors . push (match item { NestedMeta :: Meta (meta) => Error :: unknown_field_path (meta . path ()) . with_span (meta) , NestedMeta :: Lit (lit) => Error :: unexpected_expr_type (& (syn :: ExprLit { attrs : vec ! [] , lit : lit . clone () , } . into ()) ,) . with_span (lit) , }) ; } errors . finish () } }
};
}

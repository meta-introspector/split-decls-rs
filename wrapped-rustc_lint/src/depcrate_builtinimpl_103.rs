// Generated macro for impl_103 (impl)
macro_rules! Depcrate_builtinimpl_103 {
() => {
// Module: crate::builtin
// Provides: {"impl_103"}
// Dependencies: {}
impl InvalidNoMangleItems { fn check_no_mangle_on_generic_fn (& self , cx : & LateContext < '_ > , attr_span : Span , def_id : LocalDefId ,) { let generics = cx . tcx . generics_of (def_id) ; if generics . requires_monomorphization (cx . tcx) { cx . emit_span_lint (NO_MANGLE_GENERIC_ITEMS , cx . tcx . def_span (def_id) , BuiltinNoMangleGeneric { suggestion : attr_span } ,) ; } } }
};
}

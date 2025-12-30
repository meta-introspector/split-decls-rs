// Generated macro for impl_value_to_primitive (macro)
macro_rules! Depcrate_kv_valueimpl_value_to_primitive {
() => {
// Module: crate::kv::value
// Provides: {"impl_value_to_primitive"}
// Dependencies: {}
macro_rules ! impl_value_to_primitive { ($ (# [doc = $ doc : tt] $ into_name : ident -> $ into_ty : ty ,) *) => { impl <'v > Value <'v > { $ (# [doc = $ doc] pub fn $ into_name (& self) -> Option <$ into_ty > { self . inner .$ into_name () }) * } } }
};
}

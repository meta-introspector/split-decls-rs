// Generated macro for impl_to_value_nonzero_primitive (macro)
macro_rules! Depcrate_kv_valueimpl_to_value_nonzero_primitive {
() => {
// Module: crate::kv::value
// Provides: {"impl_to_value_nonzero_primitive"}
// Dependencies: {}
macro_rules ! impl_to_value_nonzero_primitive { ($ ($ into_ty : ident ,) *) => { $ (impl ToValue for std :: num ::$ into_ty { fn to_value (& self) -> Value <'_ > { Value :: from (self . get ()) } } impl <'v > From < std :: num ::$ into_ty > for Value <'v > { fn from (value : std :: num ::$ into_ty) -> Self { Value :: from (value . get ()) } } impl <'v > From <&'v std :: num ::$ into_ty > for Value <'v > { fn from (value : &'v std :: num ::$ into_ty) -> Self { Value :: from (value . get ()) } }) * } ; }
};
}

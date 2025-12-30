// Generated macro for impl_to_value_primitive (macro)
macro_rules! Depcrate_kv_valueimpl_to_value_primitive {
() => {
// Module: crate::kv::value
// Provides: {"impl_to_value_primitive"}
// Dependencies: {}
macro_rules ! impl_to_value_primitive { ($ ($ into_ty : ty ,) *) => { $ (impl ToValue for $ into_ty { fn to_value (& self) -> Value <'_ > { Value :: from (* self) } } impl <'v > From <$ into_ty > for Value <'v > { fn from (value : $ into_ty) -> Self { Value :: from_inner (value) } } impl <'v > From <&'v $ into_ty > for Value <'v > { fn from (value : &'v $ into_ty) -> Self { Value :: from_inner (* value) } }) * } ; }
};
}

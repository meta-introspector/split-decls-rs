// Generated macro for impl_composite (macro)
macro_rules! Depcrate_fieldsetsimpl_composite {
() => {
// Module: crate::fieldsets
// Provides: {"impl_composite"}
// Dependencies: {}
macro_rules ! impl_composite { ($ type : ident , $ variant : ident , $ enum : ident) => { impl $ type { # [inline] pub (crate) fn to_enum (self) -> $ enum { $ enum ::$ type (self) } } impl GetField < CompositeFieldSet > for $ type { # [inline] fn get_field (& self) -> CompositeFieldSet { CompositeFieldSet ::$ variant (self . to_enum ()) } } } ; }
};
}

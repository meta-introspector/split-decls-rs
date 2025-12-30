// Generated macro for impl_189 (impl)
macro_rules! Depcrate_internals_serialize_enumsimpl_189 {
() => {
// Module: crate::internals::serialize::enums
// Provides: {"impl_189"}
// Dependencies: {}
impl VariantFields { fn named_header (self) -> Self { let header = self . header ; VariantFields { header : quote ! { { # header .. } } , body : self . body , } } fn unnamed_header (self) -> Self { let header = self . header ; VariantFields { header : quote ! { (# header) } , body : self . body , } } }
};
}

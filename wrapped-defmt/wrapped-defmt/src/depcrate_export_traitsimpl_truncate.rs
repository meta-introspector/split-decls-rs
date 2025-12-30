// Generated macro for impl_truncate (macro)
macro_rules! Depcrate_export_traitsimpl_truncate {
() => {
// Module: crate::export::traits
// Provides: {"impl_truncate"}
// Dependencies: {}
macro_rules ! impl_truncate { ($ ($ from : ty => $ into : ty) ,*) => { $ (impl Truncate <$ into > for $ from { fn truncate (self) -> $ into { self as $ into } }) * } ; }
};
}

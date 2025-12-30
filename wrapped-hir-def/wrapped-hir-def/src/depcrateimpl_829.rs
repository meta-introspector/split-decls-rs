// Generated macro for impl_829 (impl)
macro_rules! Depcrateimpl_829 {
() => {
// Module: crate
// Provides: {"impl_829"}
// Dependencies: {}
impl EnumId { # [inline] pub fn enum_variants (self , db : & dyn DefDatabase) -> & EnumVariants { & self . enum_variants_with_diagnostics (db) . 0 } # [inline] pub fn enum_variants_with_diagnostics (self , db : & dyn DefDatabase ,) -> & (EnumVariants , Option < ThinVec < InactiveEnumVariantCode > >) { EnumVariants :: of (db , self) } }
};
}

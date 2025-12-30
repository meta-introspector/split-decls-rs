// Generated macro for impl_847 (impl)
macro_rules! Depcrateimpl_847 {
() => {
// Module: crate
// Provides: {"impl_847"}
// Dependencies: {}
impl EnumVariantId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
};
}

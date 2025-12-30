// Generated macro for impl_826 (impl)
macro_rules! Depcrateimpl_826 {
() => {
// Module: crate
// Provides: {"impl_826"}
// Dependencies: {}
impl UnionId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
};
}

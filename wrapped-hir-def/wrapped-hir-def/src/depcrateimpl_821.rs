// Generated macro for impl_821 (impl)
macro_rules! Depcrateimpl_821 {
() => {
// Module: crate
// Provides: {"impl_821"}
// Dependencies: {}
impl UnionId { pub fn fields (self , db : & dyn DefDatabase) -> & VariantFields { VariantFields :: firewall (db , self . into ()) } pub fn fields_with_source_map (self , db : & dyn DefDatabase ,) -> (Arc < VariantFields > , Arc < ExpressionStoreSourceMap >) { VariantFields :: query (db , self . into ()) } }
};
}

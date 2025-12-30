// Generated macro for impl_16 (impl)
macro_rules! Depcrate_validatorimpl_16 {
() => {
// Module: crate::validator
// Provides: {"impl_16"}
// Dependencies: {}
impl ParquetMetadata { pub fn new (features : HashMap < String , String >) -> Self { Self { features , num_rows : None , } } pub fn with_rows (mut self , num_rows : u64) -> Self { self . num_rows = Some (num_rows) ; self } }
};
}

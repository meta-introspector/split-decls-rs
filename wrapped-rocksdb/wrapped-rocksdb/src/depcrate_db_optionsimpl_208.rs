// Generated macro for impl_208 (impl)
macro_rules! Depcrate_db_optionsimpl_208 {
() => {
// Module: crate::db_options
// Provides: {"impl_208"}
// Dependencies: {}
impl OptionsMustOutliveDB { pub (crate) fn clone (& self) -> Self { Self { env : self . env . clone () , row_cache : self . row_cache . clone () , blob_cache : self . blob_cache . clone () , block_based : self . block_based . as_ref () . map (BlockBasedOptionsMustOutliveDB :: clone) , write_buffer_manager : self . write_buffer_manager . clone () , } } }
};
}

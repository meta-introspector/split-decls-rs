// Generated macro for impl_231 (impl)
macro_rules! Depcrate_inflate_streamimpl_231 {
() => {
// Module: crate::inflate::stream
// Provides: {"impl_231"}
// Dependencies: {}
impl Default for InflateState { fn default () -> Self { InflateState { decomp : DecompressorOxide :: default () , dict : [0 ; TINFL_LZ_DICT_SIZE] , dict_ofs : 0 , dict_avail : 0 , first_call : true , has_flushed : false , data_format : DataFormat :: Raw , last_status : TINFLStatus :: NeedsMoreInput , } } }
};
}

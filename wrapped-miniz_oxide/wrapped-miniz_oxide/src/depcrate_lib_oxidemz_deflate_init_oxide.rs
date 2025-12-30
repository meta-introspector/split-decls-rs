// Generated macro for mz_deflate_init_oxide (function)
macro_rules! Depcrate_lib_oxidemz_deflate_init_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_deflate_init_oxide"}
// Dependencies: {}
# [doc = " Initialize the wrapped compressor with the requested level (0-10) and default settings."] # [doc = ""] # [doc = " The compression level will be set to 6 (default) if the requested level is not available."] pub fn mz_deflate_init_oxide (stream_oxide : & mut StreamOxide < Compressor > , level : i32) -> MZResult { mz_deflate_init2_oxide (stream_oxide , level , MZ_DEFLATED , MZ_DEFAULT_WINDOW_BITS , 9 , CompressionStrategy :: Default as i32 ,) }
};
}

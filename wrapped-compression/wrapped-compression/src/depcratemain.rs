// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let sherlock_text = String :: from_utf8 (decompress_file (TEXT_SHERLOCK)) . expect ("Invalid UTF-8") ; let compressed_text = compress (& sherlock_text . repeat (20)) ; run_benchmark_group (| group | { group . register_benchmark ("brotli-compress" , | | { let mut target_buffer : Vec < u8 > = Vec :: with_capacity (1024 * 1024) ; let mut params = BrotliEncoderParams :: default () ; params . quality = 10 ; let mut text = sherlock_text . as_bytes () ; move | | { let mut writer = brotli :: CompressorWriter :: with_params (& mut target_buffer , 4096 , & params) ; std :: io :: copy (& mut text , & mut writer) . unwrap () ; writer . flush () . unwrap () ; drop (writer) ; target_buffer } }) ; group . register_benchmark ("brotli-decompress" , | | { let mut buffer : Vec < u8 > = Vec :: with_capacity (TEXT_SHERLOCK . len () * 2) ; | | { let mut reader = brotli :: Decompressor :: new (compressed_text . as_slice () , 4096) ; std :: io :: copy (& mut reader , & mut buffer) . unwrap () ; buffer } }) ; }) ; }
};
}

// Generated macro for compress (function)
macro_rules! Depcratecompress {
() => {
// Module: crate
// Provides: {"compress"}
// Dependencies: {}
fn compress (data : & str) -> Vec < u8 > { let mut target : Vec < u8 > = Vec :: with_capacity (1024 * 1024) ; let mut writer = brotli :: CompressorWriter :: with_params (& mut target , 4096 , & BrotliEncoderParams :: default ()) ; std :: io :: copy (& mut data . as_bytes () , & mut writer) . unwrap () ; writer . flush () . unwrap () ; drop (writer) ; target }
};
}

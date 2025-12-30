// Generated macro for impl_229 (impl)
macro_rules! Depcrate_enc_encoderimpl_229 {
() => {
// Module: crate::enc::encoder
// Provides: {"impl_229"}
// Dependencies: {}
impl LzmaEncoder { pub (crate) fn get_dist_slot (dist : u32) -> u32 { if dist <= DIST_MODEL_START as u32 { return dist ; } let mut n = dist ; let mut i = 31 ; if (n & 0xFFFF0000) == 0 { n <<= 16 ; i = 15 ; } if (n & 0xFF000000) == 0 { n <<= 8 ; i -= 8 ; } if (n & 0xF0000000) == 0 { n <<= 4 ; i -= 4 ; } if (n & 0xC0000000) == 0 { n <<= 2 ; i -= 2 ; } if (n & 0x80000000) == 0 { i -= 1 ; } (i << 1) + ((dist >> (i - 1)) & 1) } pub (crate) fn get_mem_usage (mode : EncodeMode , dict_size : u32 , extra_size_before : u32 , mf : MfType ,) -> u32 { let mut m = 80 ; match mode { EncodeMode :: Fast => { m += FastEncoderMode :: get_memory_usage (dict_size , extra_size_before , mf) ; } EncodeMode :: Normal => { m += NormalEncoderMode :: get_memory_usage (dict_size , extra_size_before , mf) ; } } m } }
};
}

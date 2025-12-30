// Generated macro for adler32_combine (function)
macro_rules! Depcrate_generatedadler32_combine {
() => {
// Module: crate::generated
// Provides: {"adler32_combine"}
// Dependencies: {}
# [cfg (any (zng , feature = "libc"))] pub unsafe fn adler32_combine (adler1 : z_checksum , adler2 : z_checksum , len2 : z_off_t) -> z_checksum { type Func = unsafe extern "C" fn (adler1 : z_checksum , adler2 : z_checksum , len2 : z_off_t) -> z_checksum ; let f : libloading :: Symbol < Func > = dynamic_library () . get (if_zng ! ({ concat ! ("zng_" , "adler32_combine") } , "adler32_combine") . as_bytes ()) . unwrap () ; f (adler1 , adler2 , len2) }
};
}

// Generated macro for compute_ec_symbols_size_and_pad (function)
macro_rules! Depcrate_archive_writercompute_ec_symbols_size_and_pad {
() => {
// Module: crate::archive_writer
// Provides: {"compute_ec_symbols_size_and_pad"}
// Dependencies: {}
fn compute_ec_symbols_size_and_pad (sym_map : & SymMap) -> (u64 , u64) { let mut size = size_of :: < u32 > () ; for name in sym_map . ec_map . keys () { size += size_of :: < u16 > () + name . len () + 1 ; } let mut size = u64 :: try_from (size) . unwrap () ; let pad = offset_to_alignment (size , 2) ; size += pad ; (size , pad) }
};
}

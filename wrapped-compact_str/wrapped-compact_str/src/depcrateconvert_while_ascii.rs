// Generated macro for convert_while_ascii (function)
macro_rules! Depcrateconvert_while_ascii {
() => {
// Module: crate
// Provides: {"convert_while_ascii"}
// Dependencies: {}
# [doc = " Converts the bytes while the bytes are still ascii."] # [doc = " For better average performance, this is happens in chunks of `2*size_of::<usize>()`."] # [doc = " Returns a vec with the converted bytes."] # [doc = ""] # [doc = " Copied from https://doc.rust-lang.org/nightly/src/alloc/str.rs.html#623-666"] # [inline] fn convert_while_ascii (b : & [u8] , convert : fn (& u8) -> u8) -> CompactString { let mut out = CompactString :: with_capacity (b . len ()) ; const USIZE_SIZE : usize = mem :: size_of :: < usize > () ; const MAGIC_UNROLL : usize = 2 ; const N : usize = USIZE_SIZE * MAGIC_UNROLL ; const NONASCII_MASK : usize = usize :: from_ne_bytes ([0x80 ; USIZE_SIZE]) ; let mut i = 0 ; unsafe { while i + N <= b . len () { let in_chunk = b . get_unchecked (i .. i + N) ; let out_chunk = out . spare_capacity_mut () . get_unchecked_mut (i .. i + N) ; let mut bits = 0 ; for j in 0 .. MAGIC_UNROLL { bits |= in_chunk . as_ptr () . cast :: < usize > () . add (j) . read_unaligned () ; } if bits & NONASCII_MASK != 0 { break ; } for j in 0 .. N { let out = out_chunk . get_unchecked_mut (j) ; out . write (convert (in_chunk . get_unchecked (j))) ; } i += N ; } out . set_len (i) ; } out }
};
}

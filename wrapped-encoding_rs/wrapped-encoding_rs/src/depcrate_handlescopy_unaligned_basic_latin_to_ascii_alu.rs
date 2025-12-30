// Generated macro for copy_unaligned_basic_latin_to_ascii_alu (function)
macro_rules! Depcrate_handlescopy_unaligned_basic_latin_to_ascii_alu {
() => {
// Module: crate::handles
// Provides: {"copy_unaligned_basic_latin_to_ascii_alu"}
// Dependencies: {}
# [inline (always)] fn copy_unaligned_basic_latin_to_ascii_alu < E : Endian > (src : UnalignedU16Slice , dst : & mut [u8] , offset : usize ,) -> CopyAsciiResult < usize , (u16 , usize) > { let len = :: core :: cmp :: min (src . len () , dst . len ()) ; let mut i = 0usize ; loop { if i == len { return CopyAsciiResult :: Stop (i + offset) ; } let unit = swap_if_opposite_endian :: < E > (src . at (i)) ; if unit > 0x7F { return CopyAsciiResult :: GoOn ((unit , i + offset)) ; } dst [i] = unit as u8 ; i += 1 ; } }
};
}

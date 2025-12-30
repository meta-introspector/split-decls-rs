// Generated macro for read_endianness_check (function)
macro_rules! Depcrate_util_wireread_endianness_check {
() => {
// Module: crate::util::wire
// Provides: {"read_endianness_check"}
// Dependencies: {}
# [doc = " Reads the endianness check from the beginning of the given slice and"] # [doc = " confirms that the endianness of the serialized object matches the expected"] # [doc = " endianness. If the slice is too small or if the endianness check fails,"] # [doc = " this returns an error."] # [doc = ""] # [doc = " Upon success, the total number of bytes read is returned."] pub (crate) fn read_endianness_check (slice : & [u8] ,) -> Result < usize , DeserializeError > { let (n , nr) = try_read_u32 (slice , "endianness check") ? ; assert_eq ! (nr , write_endianness_check_len ()) ; if n != 0xFEFF { return Err (DeserializeError :: endian_mismatch (0xFEFF , n)) ; } Ok (nr) }
};
}

// Generated macro for impl_68 (impl)
macro_rules! Depcrate_paramsimpl_68 {
() => {
// Module: crate::params
// Provides: {"impl_68"}
// Dependencies: {}
impl Write for Buffer { fn write_str (& mut self , input : & str) -> fmt :: Result { let bytes = input . as_bytes () ; let length = self . length as usize ; if length + bytes . len () > MAX_LENGTH { return Err (fmt :: Error) ; } self . bytes [length .. (length + bytes . len ())] . copy_from_slice (bytes) ; self . length += bytes . len () as u8 ; Ok (()) } }
};
}

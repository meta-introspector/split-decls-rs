// Generated macro for impl_120 (impl)
macro_rules! Depcrate_stringtableimpl_120 {
() => {
// Module: crate::stringtable
// Provides: {"impl_120"}
// Dependencies: {}
impl SerializableString for str { # [inline] fn serialized_size (& self) -> usize { self . len () + 1 } # [inline] fn serialize (& self , bytes : & mut [u8]) { let last_byte_index = bytes . len () - 1 ; bytes [0 .. last_byte_index] . copy_from_slice (self . as_bytes ()) ; bytes [last_byte_index] = TERMINATOR ; } }
};
}

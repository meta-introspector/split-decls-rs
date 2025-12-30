// Generated macro for SerializableString (trait)
macro_rules! Depcrate_stringtableSerializableString {
() => {
// Module: crate::stringtable
// Provides: {"SerializableString"}
// Dependencies: {}
# [doc = " Anything that implements `SerializableString` can be written to a"] # [doc = " `StringTable`."] pub trait SerializableString { fn serialized_size (& self) -> usize ; fn serialize (& self , bytes : & mut [u8]) ; }
};
}

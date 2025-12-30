// Generated macro for GnuPropertyIterator (struct)
macro_rules! Depcrate_read_elf_noteGnuPropertyIterator {
() => {
// Module: crate::read::elf::note
// Provides: {"GnuPropertyIterator"}
// Dependencies: {}
# [doc = " An iterator for the properties in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note."] # [doc = ""] # [doc = " Returned by [`Note::gnu_properties`]."] # [derive (Debug)] pub struct GnuPropertyIterator < 'data , Endian : endian :: Endian > { endian : Endian , align : usize , data : Bytes < 'data > , }
};
}

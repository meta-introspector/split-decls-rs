// Generated macro for GnuProperty (struct)
macro_rules! Depcrate_read_elf_noteGnuProperty {
() => {
// Module: crate::read::elf::note
// Provides: {"GnuProperty"}
// Dependencies: {}
# [doc = " A property in a [`elf::NT_GNU_PROPERTY_TYPE_0`] note."] # [derive (Debug)] pub struct GnuProperty < 'data > { pr_type : u32 , pr_data : & 'data [u8] , }
};
}

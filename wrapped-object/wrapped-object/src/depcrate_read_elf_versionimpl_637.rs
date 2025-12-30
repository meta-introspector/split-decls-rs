// Generated macro for impl_637 (impl)
macro_rules! Depcrate_read_elf_versionimpl_637 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_637"}
// Dependencies: {}
impl < Endian : endian :: Endian > elf :: Verneed < Endian > { # [doc = " Parse the file from the string table."] pub fn file < 'data , R : ReadRef < 'data > > (& self , endian : Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . vn_file . get (endian)) . read_error ("Invalid ELF vn_file") } }
};
}

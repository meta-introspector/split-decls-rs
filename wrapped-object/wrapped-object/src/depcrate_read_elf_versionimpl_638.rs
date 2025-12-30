// Generated macro for impl_638 (impl)
macro_rules! Depcrate_read_elf_versionimpl_638 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_638"}
// Dependencies: {}
impl < Endian : endian :: Endian > elf :: Vernaux < Endian > { # [doc = " Parse the version name from the string table."] pub fn name < 'data , R : ReadRef < 'data > > (& self , endian : Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . vna_name . get (endian)) . read_error ("Invalid ELF vna_name") } }
};
}

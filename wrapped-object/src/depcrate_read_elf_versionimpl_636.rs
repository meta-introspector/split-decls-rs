// Generated macro for impl_636 (impl)
macro_rules! Depcrate_read_elf_versionimpl_636 {
() => {
// Module: crate::read::elf::version
// Provides: {"impl_636"}
// Dependencies: {}
impl < Endian : endian :: Endian > elf :: Verdaux < Endian > { # [doc = " Parse the version name from the string table."] pub fn name < 'data , R : ReadRef < 'data > > (& self , endian : Endian , strings : StringTable < 'data , R > ,) -> Result < & 'data [u8] > { strings . get (self . vda_name . get (endian)) . read_error ("Invalid ELF vda_name") } }
};
}

// Generated macro for print_section_relr (function)
macro_rules! Depcrate_readobj_elfprint_section_relr {
() => {
// Module: crate::readobj::elf
// Provides: {"print_section_relr"}
// Dependencies: {}
fn print_section_relr < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , section : & Elf :: SectionHeader ,) { if ! p . options . relocations { return ; } if let Some (Some (relocations)) = section . relr (endian , data) . print_err (p) { for relocation in relocations { p . field_hex ("Offset" , relocation . into ()) ; } } }
};
}

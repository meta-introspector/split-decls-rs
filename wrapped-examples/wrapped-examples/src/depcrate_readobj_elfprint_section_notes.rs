// Generated macro for print_section_notes (function)
macro_rules! Depcrate_readobj_elfprint_section_notes {
() => {
// Module: crate::readobj::elf
// Provides: {"print_section_notes"}
// Dependencies: {}
fn print_section_notes < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , elf : & Elf , section : & Elf :: SectionHeader ,) { if ! p . options . elf_notes { return ; } if let Some (Some (notes)) = section . notes (endian , data) . print_err (p) { print_notes (p , endian , elf , notes) ; } }
};
}

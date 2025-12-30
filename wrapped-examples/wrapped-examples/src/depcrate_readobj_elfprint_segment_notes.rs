// Generated macro for print_segment_notes (function)
macro_rules! Depcrate_readobj_elfprint_segment_notes {
() => {
// Module: crate::readobj::elf
// Provides: {"print_segment_notes"}
// Dependencies: {}
fn print_segment_notes < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , elf : & Elf , segment : & Elf :: ProgramHeader ,) { if ! p . options . elf_notes { return ; } if let Some (Some (notes)) = segment . notes (endian , data) . print_err (p) { print_notes (p , endian , elf , notes) ; } }
};
}

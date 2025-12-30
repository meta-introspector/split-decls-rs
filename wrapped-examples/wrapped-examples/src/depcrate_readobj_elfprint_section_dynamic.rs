// Generated macro for print_section_dynamic (function)
macro_rules! Depcrate_readobj_elfprint_section_dynamic {
() => {
// Module: crate::readobj::elf
// Provides: {"print_section_dynamic"}
// Dependencies: {}
fn print_section_dynamic < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if ! p . options . elf_dynamic { return ; } if let Some (Some ((dynamic , index))) = section . dynamic (endian , data) . print_err (p) { let strings = sections . strings (endian , data , index) . unwrap_or_default () ; print_dynamic (p , endian , elf , dynamic , strings) ; } }
};
}

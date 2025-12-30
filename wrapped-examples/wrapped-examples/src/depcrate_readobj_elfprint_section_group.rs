// Generated macro for print_section_group (function)
macro_rules! Depcrate_readobj_elfprint_section_group {
() => {
// Module: crate::readobj::elf
// Provides: {"print_section_group"}
// Dependencies: {}
fn print_section_group < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , _elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if let Some (Some ((flag , members))) = section . group (endian , data) . print_err (p) { p . field_enum ("GroupFlag" , flag , FLAGS_GRP) ; p . group ("GroupSections" , | p | { for member in members { let index = member . get (endian) ; p . print_indent () ; if let Some (section) = sections . section (SectionIndex (index as usize)) . print_err (p) { if let Some (name) = sections . section_name (endian , section) . print_err (p) { p . print_string (name) ; writeln ! (p . w , " ({})" , index) . unwrap () ; } else { writeln ! (p . w , "{}" , index) . unwrap () ; } } else { writeln ! (p . w , "{}" , index) . unwrap () ; } } }) ; } }
};
}

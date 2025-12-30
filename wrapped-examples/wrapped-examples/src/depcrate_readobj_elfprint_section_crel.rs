// Generated macro for print_section_crel (function)
macro_rules! Depcrate_readobj_elfprint_section_crel {
() => {
// Module: crate::readobj::elf
// Provides: {"print_section_crel"}
// Dependencies: {}
fn print_section_crel < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if ! p . options . relocations { return ; } if let Some (Some ((relocations , link))) = section . crel (endian , data) . print_err (p) { let symbols = if link . 0 != 0 { sections . symbol_table_by_index (endian , data , link) . print_err (p) } else { None } ; let proc = rel_flag_type (endian , elf) ; for relocation_result in relocations { let Some (relocation) = relocation_result . print_err (p) else { return ; } ; p . group ("Relocation" , | p | { p . field_hex ("Offset" , relocation . r_offset) ; p . field_enum ("Type" , relocation . r_type , proc) ; print_rel_symbol (p , endian , symbols , relocation . symbol ()) ; let addend = relocation . r_addend ; if addend != 0 { p . field_hex ("Addend" , addend) ; } }) ; } } }
};
}

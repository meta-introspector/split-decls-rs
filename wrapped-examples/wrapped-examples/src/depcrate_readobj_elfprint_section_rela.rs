// Generated macro for print_section_rela (function)
macro_rules! Depcrate_readobj_elfprint_section_rela {
() => {
// Module: crate::readobj::elf
// Provides: {"print_section_rela"}
// Dependencies: {}
fn print_section_rela < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , elf : & Elf , sections : & SectionTable < Elf > , section : & Elf :: SectionHeader ,) { if ! p . options . relocations { return ; } if let Some (Some ((relocations , link))) = section . rela (endian , data) . print_err (p) { let symbols = if link . 0 != 0 { sections . symbol_table_by_index (endian , data , link) . print_err (p) } else { None } ; let proc = rel_flag_type (endian , elf) ; for relocation in relocations { p . group ("Relocation" , | p | { p . field_hex ("Offset" , relocation . r_offset (endian) . into ()) ; p . field_enum ("Type" , relocation . r_type (endian , elf . is_mips64el (endian)) , proc ,) ; let sym = relocation . symbol (endian , elf . is_mips64el (endian)) ; print_rel_symbol (p , endian , symbols , sym) ; let addend = relocation . r_addend (endian) . into () ; if addend != 0 { p . field_hex ("Addend" , addend) ; } }) ; } } }
};
}

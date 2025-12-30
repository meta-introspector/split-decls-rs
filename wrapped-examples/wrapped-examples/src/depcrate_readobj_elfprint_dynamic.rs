// Generated macro for print_dynamic (function)
macro_rules! Depcrate_readobj_elfprint_dynamic {
() => {
// Module: crate::readobj::elf
// Provides: {"print_dynamic"}
// Dependencies: {}
fn print_dynamic < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , elf : & Elf , dynamic : & [Elf :: Dyn] , dynstr : StringTable ,) { let proc = match elf . e_machine (endian) { EM_SPARC => FLAGS_DT_SPARC , EM_MIPS => FLAGS_DT_MIPS , EM_ALPHA => FLAGS_DT_ALPHA , EM_PPC => FLAGS_DT_PPC , EM_PPC64 => FLAGS_DT_PPC64 , EM_IA_64 => FLAGS_DT_IA_64 , EM_AARCH64 => FLAGS_DT_AARCH64 , EM_ALTERA_NIOS2 => FLAGS_DT_NIOS2 , EM_RISCV => FLAGS_DT_RISCV , _ => & [] , } ; for d in dynamic { let tag = d . d_tag (endian) . into () ; let val = d . d_val (endian) . into () ; p . group ("Dynamic" , | p | { if let Some (tag) = d . tag32 (endian) { p . field_enums ("Tag" , tag , & [FLAGS_DT , proc]) ; if d . is_string (endian) { p . field_string ("Value" , val , d . string (endian , dynstr)) ; } else { p . field_hex ("Value" , val) ; if tag == DT_FLAGS { p . flags (val , 0 , FLAGS_DF) ; } else if tag == DT_FLAGS_1 { p . flags (val , 0 , FLAGS_DF_1) ; } } } else { p . field_hex ("Tag" , tag) ; p . field_hex ("Value" , val) ; } }) ; if tag == DT_NULL . into () { break ; } } }
};
}

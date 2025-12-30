// Generated macro for print_segment_dynamic (function)
macro_rules! Depcrate_readobj_elfprint_segment_dynamic {
() => {
// Module: crate::readobj::elf
// Provides: {"print_segment_dynamic"}
// Dependencies: {}
fn print_segment_dynamic < Elf : FileHeader > (p : & mut Printer < '_ > , endian : Elf :: Endian , data : & [u8] , elf : & Elf , segments : & [Elf :: ProgramHeader] , segment : & Elf :: ProgramHeader ,) { if ! p . options . elf_dynamic { return ; } if let Some (Some (dynamic)) = segment . dynamic (endian , data) . print_err (p) { let mut strtab = 0 ; let mut strsz = 0 ; for d in dynamic { let tag = d . d_tag (endian) . into () ; if tag == DT_STRTAB . into () { strtab = d . d_val (endian) . into () ; } else if tag == DT_STRSZ . into () { strsz = d . d_val (endian) . into () ; } } let mut dynstr = StringTable :: default () ; for s in segments { if let Ok (Some (data)) = s . data_range (endian , data , strtab , strsz) { dynstr = StringTable :: new (data , 0 , data . len () as u64) ; break ; } } print_dynamic (p , endian , elf , dynamic , dynstr) ; } }
};
}

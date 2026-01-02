mkuse!{use super :: mem ;}
mkuse!{use crate :: slice :: from_raw_parts ;}
mkitem!{const R_X86_64_RELATIVE : u32 = 8 ;}
mkitem!{mkstruct!{# [repr (packed)] struct Rela < T > { offset : T , info : T , addend : T , }}}

macro_rules! relocate_elf_rela_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function relocate_elf_rela in module {}", module_path!());
    };
}

mkfn!{
    relocate_elf_rela_introspect!();
    pub fn relocate_elf_rela () { unsafe extern "C" { static RELA : u64 ; static RELACOUNT : usize ; } if unsafe { RELACOUNT } == 0 { return ; } let relas = unsafe { from_raw_parts :: < Rela < u64 > > (mem :: rel_ptr (RELA) , RELACOUNT) } ; for rela in relas { if rela . info != (R_X86_64_RELATIVE as u64) { rtabort ! ("Invalid relocation") ; } unsafe { * mem :: rel_ptr_mut :: < * const () > (rela . offset) = mem :: rel_ptr (rela . addend) } ; } }
}
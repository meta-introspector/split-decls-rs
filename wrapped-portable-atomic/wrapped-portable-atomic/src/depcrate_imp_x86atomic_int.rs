// Generated macro for atomic_int (macro)
macro_rules! Depcrate_imp_x86atomic_int {
() => {
// Module: crate::imp::x86
// Provides: {"atomic_int"}
// Dependencies: {}
macro_rules ! atomic_int { ($ atomic_type : ident , $ ptr_size : tt) => { impl $ atomic_type { # [inline] pub (crate) fn not (& self , _order : Ordering) { let dst = self . as_ptr () ; unsafe { asm ! (concat ! ("lock not " , $ ptr_size , " ptr [{dst" , ptr_modifier ! () , "}]") , dst = in (reg) dst , options (nostack , preserves_flags) ,) ; } } # [inline] pub (crate) fn neg (& self , _order : Ordering) { let dst = self . as_ptr () ; unsafe { asm ! (concat ! ("lock neg " , $ ptr_size , " ptr [{dst" , ptr_modifier ! () , "}]") , dst = in (reg) dst , options (nostack) ,) ; } } } } ; }
};
}

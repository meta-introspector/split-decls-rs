// Generated macro for atomic8 (macro)
macro_rules! Depcrate_imp_avratomic8 {
() => {
// Module: crate::imp::avr
// Provides: {"atomic8"}
// Dependencies: {}
macro_rules ! atomic8 { ($ atomic_type : ident , $ value_type : ty) => { # [repr (transparent)] pub (crate) struct $ atomic_type { v : UnsafeCell <$ value_type >, } unsafe impl Send for $ atomic_type { } unsafe impl Sync for $ atomic_type { } impl $ atomic_type { # [inline] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub (crate) fn load (& self , order : Ordering) -> $ value_type { crate :: utils :: assert_load_ordering (order) ; let src = self . v . get () ; unsafe { let out ; asm ! ("ld {out}, Z" , in ("Z") src , out = out (reg) out , options (nostack , preserves_flags) ,) ; out } } # [inline] # [cfg_attr (all (debug_assertions , not (portable_atomic_no_track_caller)) , track_caller)] pub (crate) fn store (& self , val : $ value_type , order : Ordering) { crate :: utils :: assert_store_ordering (order) ; let dst = self . v . get () ; unsafe { asm ! ("st Z, {val}" , in ("Z") dst , val = in (reg) val , options (nostack , preserves_flags) ,) ; } } } } ; }
};
}

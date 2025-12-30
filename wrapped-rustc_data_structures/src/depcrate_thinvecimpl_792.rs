// Generated macro for impl_792 (impl)
macro_rules! Depcrate_thinvecimpl_792 {
() => {
// Module: crate::thinvec
// Provides: {"impl_792"}
// Dependencies: {}
impl < 'a , T , F > ExtractIf < 'a , T , F > where F : FnMut (& mut T) -> bool , { pub fn new (vec : & 'a mut ThinVec < T > , filter : F) -> Self { let old_len = vec . len () ; unsafe { vec . set_len (0) ; } ExtractIf { vec , idx : 0 , del : 0 , old_len , pred : filter } } }
};
}

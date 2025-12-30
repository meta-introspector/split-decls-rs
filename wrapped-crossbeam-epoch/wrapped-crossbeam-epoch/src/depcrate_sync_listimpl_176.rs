// Generated macro for impl_176 (impl)
macro_rules! Depcrate_sync_listimpl_176 {
() => {
// Module: crate::sync::list
// Provides: {"impl_176"}
// Dependencies: {}
impl < T , C : IsElement < T > > Drop for List < T , C > { fn drop (& mut self) { unsafe { let guard = unprotected () ; let mut curr = self . head . load (Relaxed , guard) ; while let Some (c) = curr . as_ref () { let succ = c . next . load (Relaxed , guard) ; assert_eq ! (succ . tag () , 1) ; C :: finalize (curr . deref () , guard) ; curr = succ ; } } } }
};
}

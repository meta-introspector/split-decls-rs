// Generated macro for impl_49 (impl)
macro_rules! Depcrate_adaptors_coalesceimpl_49 {
() => {
// Module: crate::adaptors::coalesce
// Provides: {"impl_49"}
// Dependencies: {}
impl < I , F , C > FusedIterator for CoalesceBy < I , F , C > where I : Iterator , F : CoalescePredicate < I :: Item , C :: CItem > , C : CountItem < I :: Item > , { }
};
}

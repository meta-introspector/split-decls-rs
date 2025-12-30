// Generated macro for impl_253 (impl)
macro_rules! Depcrate_combinationsimpl_253 {
() => {
// Module: crate::combinations
// Provides: {"impl_253"}
// Dependencies: {}
impl < I , Idx > Iterator for CombinationsGeneric < I , Idx > where I : Iterator , I :: Item : Clone , Idx : PoolIndex < I :: Item > , { type Item = Idx :: Item ; fn next (& mut self) -> Option < Self :: Item > { let done = if self . first { self . init () } else { self . increment_indices () } ; if done { return None ; } Some (self . indices . extract_item (& self . pool)) } fn nth (& mut self , n : usize) -> Option < Self :: Item > { self . try_nth (n) . ok () } fn size_hint (& self) -> (usize , Option < usize >) { let (mut low , mut upp) = self . pool . size_hint () ; low = remaining_for (low , self . first , self . indices . borrow ()) . unwrap_or (usize :: MAX) ; upp = upp . and_then (| upp | remaining_for (upp , self . first , self . indices . borrow ())) ; (low , upp) } # [inline] fn count (self) -> usize { self . n_and_count () . 1 } }
};
}

// Generated macro for impl_171 (impl)
macro_rules! Depcrate_vec_spliceimpl_171 {
() => {
// Module: crate::vec::splice
// Provides: {"impl_171"}
// Dependencies: {}
impl < I : Iterator , A : Allocator > Drop for Splice < '_ , I , A > { # [inline] fn drop (& mut self) { self . drain . by_ref () . for_each (drop) ; unsafe { if self . drain . tail_len == 0 { self . drain . vec . as_mut () . extend (self . replace_with . by_ref ()) ; return ; } if ! self . drain . fill (& mut self . replace_with) { return ; } let (lower_bound , _upper_bound) = self . replace_with . size_hint () ; if lower_bound > 0 { self . drain . move_tail (lower_bound) ; if ! self . drain . fill (& mut self . replace_with) { return ; } } let mut collected = self . replace_with . by_ref () . collect :: < Vec < I :: Item > > () . into_iter () ; if collected . len () > 0 { self . drain . move_tail (collected . len ()) ; let filled = self . drain . fill (& mut collected) ; debug_assert ! (filled) ; debug_assert_eq ! (collected . len () , 0) ; } } } }
};
}

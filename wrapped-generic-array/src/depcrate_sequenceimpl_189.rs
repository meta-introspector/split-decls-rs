// Generated macro for impl_189 (impl)
macro_rules! Depcrate_sequenceimpl_189 {
() => {
// Module: crate::sequence
// Provides: {"impl_189"}
// Dependencies: {}
unsafe impl < T , N > Remove < T , N > for GenericArray < T , N > where N : ArrayLength + Sub < B1 > , Sub1 < N > : ArrayLength , { type Output = GenericArray < T , Sub1 < N > > ; # [inline] unsafe fn remove_unchecked (self , idx : usize) -> (T , Self :: Output) { if idx >= N :: USIZE || N :: USIZE == 0 { core :: hint :: unreachable_unchecked () ; } let mut array = ManuallyDrop :: new (self) ; let dst = array . as_mut_ptr () . add (idx) ; let removed = ptr :: read (dst) ; ptr :: copy (dst . add (1) , dst , N :: USIZE - idx - 1) ; (removed , mem :: transmute_copy (& array)) } # [inline] unsafe fn swap_remove_unchecked (self , idx : usize) -> (T , Self :: Output) { if idx >= N :: USIZE || N :: USIZE == 0 { core :: hint :: unreachable_unchecked () ; } let mut array = ManuallyDrop :: new (self) ; array . swap (idx , N :: USIZE - 1) ; let removed = ptr :: read (array . as_ptr () . add (N :: USIZE - 1)) ; (removed , mem :: transmute_copy (& array)) } }
};
}

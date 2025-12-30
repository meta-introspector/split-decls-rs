// Generated macro for impl_183 (impl)
macro_rules! Depcrate_sequenceimpl_183 {
() => {
// Module: crate::sequence
// Provides: {"impl_183"}
// Dependencies: {}
unsafe impl < T , N , K > Split < T , K > for GenericArray < T , N > where N : ArrayLength , K : ArrayLength , N : Sub < K > , Diff < N , K > : ArrayLength , { type First = GenericArray < T , K > ; type Second = GenericArray < T , Diff < N , K > > ; # [inline] fn split (self) -> (Self :: First , Self :: Second) { unsafe { let whole = ManuallyDrop :: new (self) ; let head = ptr :: read (whole . as_ptr () as * const _) ; let tail = ptr :: read (whole . as_ptr () . add (K :: USIZE) as * const _) ; (head , tail) } } }
};
}

// Generated macro for impl_187 (impl)
macro_rules! Depcrate_sequenceimpl_187 {
() => {
// Module: crate::sequence
// Provides: {"impl_187"}
// Dependencies: {}
unsafe impl < T , N , M > Concat < T , M > for GenericArray < T , N > where N : ArrayLength + Add < M > , M : ArrayLength , Sum < N , M > : ArrayLength , { type Rest = GenericArray < T , M > ; type Output = GenericArray < T , Sum < N , M > > ; # [inline] fn concat (self , rest : Self :: Rest) -> Self :: Output { let mut output : MaybeUninit < Self :: Output > = MaybeUninit :: uninit () ; let out_ptr = output . as_mut_ptr () as * mut Self ; unsafe { ptr :: write (out_ptr , self) ; ptr :: write (out_ptr . add (1) as * mut _ , rest) ; output . assume_init () } } }
};
}

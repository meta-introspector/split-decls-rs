// Generated macro for impl_180 (impl)
macro_rules! Depcrate_sequenceimpl_180 {
() => {
// Module: crate::sequence
// Provides: {"impl_180"}
// Dependencies: {}
unsafe impl < T , N : ArrayLength > Lengthen < T > for GenericArray < T , N > where N : Add < B1 > , Add1 < N > : ArrayLength , Add1 < N > : Sub < B1 , Output = N > , Sub1 < Add1 < N > > : ArrayLength , { type Longer = GenericArray < T , Add1 < N > > ; # [inline] fn append (self , last : T) -> Self :: Longer { let mut longer : MaybeUninit < Self :: Longer > = MaybeUninit :: uninit () ; let out_ptr = longer . as_mut_ptr () as * mut Self ; unsafe { ptr :: write (out_ptr , self) ; ptr :: write (out_ptr . add (1) as * mut T , last) ; longer . assume_init () } } # [inline] fn prepend (self , first : T) -> Self :: Longer { let mut longer : MaybeUninit < Self :: Longer > = MaybeUninit :: uninit () ; let out_ptr = longer . as_mut_ptr () as * mut T ; unsafe { ptr :: write (out_ptr , first) ; ptr :: write (out_ptr . add (1) as * mut Self , self) ; longer . assume_init () } } }
};
}

// Generated macro for deref_impl (macro)
macro_rules! Depcrate_base_coordinatesderef_impl {
() => {
// Module: crate::base::coordinates
// Provides: {"deref_impl"}
// Dependencies: {}
macro_rules ! deref_impl (($ R : ty , $ C : ty ; $ Target : ident) => { impl < T : Scalar , S > Deref for Matrix < T , $ R , $ C , S > where S : RawStorage < T , $ R , $ C > + IsContiguous { type Target = $ Target < T >; # [inline] fn deref (& self) -> & Self :: Target { unsafe { &* (self . data . ptr () as * const Self :: Target) } } } impl < T : Scalar , S > DerefMut for Matrix < T , $ R , $ C , S > where S : RawStorageMut < T , $ R , $ C > + IsContiguous { # [inline] fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { & mut * (self . data . ptr_mut () as * mut Self :: Target) } } } }) ;
};
}

// Generated macro for impl_70 (impl)
macro_rules! Depcrate_valueimpl_70 {
() => {
// Module: crate::value
// Provides: {"impl_70"}
// Dependencies: {}
impl < 'sval > Value < 'sval > { # [doc = "\n    Buffer a value.\n\n    This method will fail if the `alloc` feature is not enabled.\n    "] pub fn collect (v : & 'sval (impl sval :: Value + ? Sized)) -> Result < Self , Error > { ValueBuf :: collect (v) . map (| buf | buf . into_value ()) } # [doc = "\n    Fully buffer this value, including any internal borrowed data.\n\n    This method will fail if the `alloc` feature is not enabled.\n    "] pub fn into_owned (self) -> Result < Value < 'static > , Error > { # [cfg (feature = "alloc")] { let Value { mut parts , _marker } = self ; for part in parts . iter_mut () { crate :: assert_static (part . into_owned_in_place ()) ; } let mut parts = unsafe { mem :: transmute :: < Buf < ValuePart < 'sval > , 1 > , Buf < ValuePart < 'static > , 1 > > (parts) } ; crate :: assert_static (& mut parts) ; Ok (Value { parts , _marker : PhantomData , }) } # [cfg (not (feature = "alloc"))] { Err (Error :: no_alloc ("owned value")) } } }
};
}

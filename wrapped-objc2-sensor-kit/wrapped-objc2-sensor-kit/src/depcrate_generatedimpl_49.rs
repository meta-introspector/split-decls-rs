// Generated macro for impl_49 (impl)
macro_rules! Depcrate_generatedimpl_49 {
() => {
// Module: crate::generated
// Provides: {"impl_49"}
// Dependencies: {}
impl < SampleType : ? Sized + Message > SRFetchResult < SampleType > { # [doc = " Unchecked conversion of the generic parameter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The generic must be valid to reinterpret as the given type."] # [inline] pub unsafe fn cast_unchecked < NewSampleType : ? Sized + Message > (& self ,) -> & SRFetchResult < NewSampleType > { unsafe { & * ((self as * const Self) . cast ()) } } }
};
}

// Generated macro for impl_179 (impl)
macro_rules! Depcrate_generatedimpl_179 {
() => {
// Module: crate::generated
// Provides: {"impl_179"}
// Dependencies: {}
impl < UnitType : ? Sized + Message + AsRef < NSUnit > > SRKeyboardProbabilityMetric < UnitType > { # [doc = " Unchecked conversion of the generic parameter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The generic must be valid to reinterpret as the given type."] # [inline] pub unsafe fn cast_unchecked < NewUnitType : ? Sized + Message + AsRef < NSUnit > > (& self ,) -> & SRKeyboardProbabilityMetric < NewUnitType > { unsafe { & * ((self as * const Self) . cast ()) } } }
};
}

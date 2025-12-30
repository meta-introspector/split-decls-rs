// Generated macro for impl_725 (impl)
macro_rules! Depcrate_offset_fixedimpl_725 {
() => {
// Module: crate::offset::fixed
// Provides: {"impl_725"}
// Dependencies: {}
# [cfg (all (feature = "arbitrary" , feature = "std"))] impl arbitrary :: Arbitrary < '_ > for FixedOffset { fn arbitrary (u : & mut arbitrary :: Unstructured) -> arbitrary :: Result < FixedOffset > { let secs = u . int_in_range (- 86_399 ..= 86_399) ? ; let fixed_offset = FixedOffset :: east_opt (secs) . expect ("Could not generate a valid chrono::FixedOffset. It looks like implementation of Arbitrary for FixedOffset is erroneous.") ; Ok (fixed_offset) } }
};
}

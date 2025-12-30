// Generated macro for impl_493 (impl)
macro_rules! Depcrate_arbitrary__std_stringimpl_493 {
() => {
// Module: crate::arbitrary::_std::string
// Provides: {"impl_493"}
// Dependencies: {}
impl Arbitrary for String { type Parameters = StringParam ; type Strategy = & 'static str ; # [doc = " ## Panics"] # [doc = ""] # [doc = " This implementation panics if the input is not a valid regex proptest"] # [doc = " can handle."] fn arbitrary_with (args : Self :: Parameters) -> Self :: Strategy { args . into () } }
};
}

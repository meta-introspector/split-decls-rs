// Generated macro for impl_154 (impl)
macro_rules! Depcrate_internal_test_outcomeimpl_154 {
() => {
// Module: crate::internal::test_outcome
// Provides: {"impl_154"}
// Dependencies: {}
impl TestAssertionFailure { # [doc = " Creates a new instance with the given `description`."] # [doc = ""] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [track_caller] pub fn create (description : String) -> Self { Self { description , custom_message : None , location : Location :: Real (std :: panic :: Location :: caller ()) , } } # [doc = " Set `location`` to a fake value."] # [doc = ""] # [doc = " **For internal use only. API stablility is not guaranteed!**"] pub fn with_fake_location (mut self , file : & 'static str , line : u32 , column : u32) -> Self { self . location = Location :: Fake { file , line , column } ; self } pub (crate) fn log (& self) { TestOutcome :: fail_current_test () ; println ! ("{self}") ; } }
};
}

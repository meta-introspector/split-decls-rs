// Generated macro for Location (enum)
macro_rules! Depcrate_internal_test_outcomeLocation {
() => {
// Module: crate::internal::test_outcome
// Provides: {"Location"}
// Dependencies: {}
# [doc = " A code location."] # [doc = ""] # [doc = " `std::panic::Location` does not provide a constructor, hence we cannot"] # [doc = " construct a fake value."] # [doc = ""] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [doc (hidden)] # [derive (Clone)] enum Location { Real (& 'static std :: panic :: Location < 'static >) , Fake { file : & 'static str , line : u32 , column : u32 } , }
};
}

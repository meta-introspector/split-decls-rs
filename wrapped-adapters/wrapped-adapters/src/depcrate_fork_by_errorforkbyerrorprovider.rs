// Generated macro for ForkByErrorProvider (struct)
macro_rules! Depcrate_fork_by_errorForkByErrorProvider {
() => {
// Module: crate::fork::by_error
// Provides: {"ForkByErrorProvider"}
// Dependencies: {}
# [doc = " A provider that returns data from one of two child providers based on a predicate function."] # [doc = ""] # [doc = " This is an abstract forking provider that must be provided with a type implementing the"] # [doc = " [`ForkByErrorPredicate`] trait."] # [doc = ""] # [doc = " [`ForkByErrorProvider`] does not support forking between [`DataProvider`]s. However, it"] # [doc = " supports forking between [`BufferProvider`], and [`DynamicDataProvider`]."] # [derive (Debug , PartialEq , Eq)] pub struct ForkByErrorProvider < P0 , P1 , F > (P0 , P1 , F) ;
};
}

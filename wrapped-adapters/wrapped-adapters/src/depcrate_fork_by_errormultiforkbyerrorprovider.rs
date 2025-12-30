// Generated macro for MultiForkByErrorProvider (struct)
macro_rules! Depcrate_fork_by_errorMultiForkByErrorProvider {
() => {
// Module: crate::fork::by_error
// Provides: {"MultiForkByErrorProvider"}
// Dependencies: {}
# [doc = " A provider that returns data from the first child provider passing a predicate function."] # [doc = ""] # [doc = " This is an abstract forking provider that must be provided with a type implementing the"] # [doc = " [`ForkByErrorPredicate`] trait."] # [doc = ""] # [doc = " [`MultiForkByErrorProvider`] does not support forking between [`DataProvider`]s. However, it"] # [doc = " supports forking between [`BufferProvider`], and [`DynamicDataProvider`]."] # [derive (Debug)] pub struct MultiForkByErrorProvider < P , F > { providers : Vec < P > , predicate : F , }
};
}

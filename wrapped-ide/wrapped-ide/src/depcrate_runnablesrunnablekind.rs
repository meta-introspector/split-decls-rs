// Generated macro for RunnableKind (enum)
macro_rules! Depcrate_runnablesRunnableKind {
() => {
// Module: crate::runnables
// Provides: {"RunnableKind"}
// Dependencies: {}
# [derive (Debug , Clone , Hash , PartialEq , Eq)] pub enum RunnableKind { TestMod { path : String } , Test { test_id : TestId , attr : TestAttr } , Bench { test_id : TestId } , DocTest { test_id : TestId } , Bin , }
};
}

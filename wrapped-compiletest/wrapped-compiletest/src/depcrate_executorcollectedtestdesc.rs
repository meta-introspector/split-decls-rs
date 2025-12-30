// Generated macro for CollectedTestDesc (struct)
macro_rules! Depcrate_executorCollectedTestDesc {
() => {
// Module: crate::executor
// Provides: {"CollectedTestDesc"}
// Dependencies: {}
# [doc = " Information that was historically needed to create a libtest `TestDesc`."] pub (crate) struct CollectedTestDesc { pub (crate) name : String , pub (crate) ignore : bool , pub (crate) ignore_message : Option < Cow < 'static , str > > , pub (crate) should_panic : ShouldPanic , }
};
}

// Generated macro for impl_64 (impl)
macro_rules! Depcrate_scenarios_structsimpl_64 {
() => {
// Module: crate::scenarios::structs
// Provides: {"impl_64"}
// Dependencies: {}
impl Query { pub fn new < K : Into < L10nKey > > (input : K , output : Option < L10nMessage >) -> Self { Self { input : input . into () , output , exceptional_context : ExceptionalContext :: None , } } }
};
}

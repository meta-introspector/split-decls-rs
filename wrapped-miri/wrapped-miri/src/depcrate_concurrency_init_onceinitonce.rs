// Generated macro for InitOnce (struct)
macro_rules! Depcrate_concurrency_init_onceInitOnce {
() => {
// Module: crate::concurrency::init_once
// Provides: {"InitOnce"}
// Dependencies: {}
# [doc = " The one time initialization state."] # [derive (Default , Debug)] pub (super) struct InitOnce { status : InitOnceStatus , waiters : VecDeque < ThreadId > , clock : VClock , }
};
}

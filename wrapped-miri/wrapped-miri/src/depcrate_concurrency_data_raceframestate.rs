// Generated macro for FrameState (struct)
macro_rules! Depcrate_concurrency_data_raceFrameState {
() => {
// Module: crate::concurrency::data_race
// Provides: {"FrameState"}
// Dependencies: {}
# [doc = " Vector clock state for a stack frame (tracking the local variables"] # [doc = " that do not have an allocation yet)."] # [derive (Debug , Default)] pub struct FrameState { local_clocks : RefCell < FxHashMap < mir :: Local , LocalClocks > > , }
};
}

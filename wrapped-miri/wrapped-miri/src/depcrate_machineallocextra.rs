// Generated macro for AllocExtra (struct)
macro_rules! Depcrate_machineAllocExtra {
() => {
// Module: crate::machine
// Provides: {"AllocExtra"}
// Dependencies: {}
# [doc = " Extra per-allocation data"] # [derive (Debug)] pub struct AllocExtra < 'tcx > { # [doc = " Global state of the borrow tracker, if enabled."] pub borrow_tracker : Option < borrow_tracker :: AllocState > , # [doc = " Extra state for data race detection."] # [doc = ""] # [doc = " Invariant: The enum variant must match the enum variant in the `data_race` field on `MiriMachine`"] pub data_race : AllocDataRaceHandler , # [doc = " A backtrace to where this allocation was allocated."] # [doc = " As this is recorded for leak reports, it only exists"] # [doc = " if this allocation is leakable. The backtrace is not"] # [doc = " pruned yet; that should be done before printing it."] pub backtrace : Option < Vec < FrameInfo < 'tcx > > > , # [doc = " Synchronization primitives like to attach extra data to particular addresses. We store that"] # [doc = " inside the relevant allocation, to ensure that everything is removed when the allocation is"] # [doc = " freed."] # [doc = " This maps offsets to synchronization-primitive-specific data."] pub sync : FxHashMap < Size , Box < dyn Any > > , }
};
}

// Generated macro for FrameExtra (struct)
macro_rules! Depcrate_machineFrameExtra {
() => {
// Module: crate::machine
// Provides: {"FrameExtra"}
// Dependencies: {}
# [doc = " Extra data stored with each stack frame"] pub struct FrameExtra < 'tcx > { # [doc = " Extra data for the Borrow Tracker."] pub borrow_tracker : Option < borrow_tracker :: FrameState > , # [doc = " If this is Some(), then this is a special \"catch unwind\" frame (the frame of `try_fn`"] # [doc = " called by `try`). When this frame is popped during unwinding a panic,"] # [doc = " we stop unwinding, use the `CatchUnwindData` to handle catching."] pub catch_unwind : Option < CatchUnwindData < 'tcx > > , # [doc = " If `measureme` profiling is enabled, holds timing information"] # [doc = " for the start of this frame. When we finish executing this frame,"] # [doc = " we use this to register a completed event with `measureme`."] pub timing : Option < measureme :: DetachedTiming > , # [doc = " Indicates whether a `Frame` is part of a workspace-local crate and is also not"] # [doc = " `#[track_caller]`. We compute this once on creation and store the result, as an"] # [doc = " optimization."] # [doc = " This is used by `MiriMachine::current_span` and `MiriMachine::caller_span`"] pub is_user_relevant : bool , # [doc = " Data race detector per-frame data."] pub data_race : Option < data_race :: FrameState > , }
};
}

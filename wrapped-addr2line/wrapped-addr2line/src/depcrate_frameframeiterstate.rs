// Generated macro for FrameIterState (enum)
macro_rules! Depcrate_frameFrameIterState {
() => {
// Module: crate::frame
// Provides: {"FrameIterState"}
// Dependencies: {}
enum FrameIterState < 'ctx , R > where R : gimli :: Reader , { Empty , Location (Option < Location < 'ctx > >) , Frames (FrameIterFrames < 'ctx , R >) , }
};
}

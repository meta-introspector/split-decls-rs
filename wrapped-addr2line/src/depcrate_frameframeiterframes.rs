// Generated macro for FrameIterFrames (struct)
macro_rules! Depcrate_frameFrameIterFrames {
() => {
// Module: crate::frame
// Provides: {"FrameIterFrames"}
// Dependencies: {}
struct FrameIterFrames < 'ctx , R > where R : gimli :: Reader , { unit : & 'ctx ResUnit < R > , sections : & 'ctx gimli :: Dwarf < R > , function : & 'ctx Function < R > , inlined_functions : iter :: Rev < maybe_small :: IntoIter < & 'ctx InlinedFunction < R > > > , next : Option < Location < 'ctx > > , }
};
}

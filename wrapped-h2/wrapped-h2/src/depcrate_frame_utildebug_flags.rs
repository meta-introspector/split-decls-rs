// Generated macro for debug_flags (function)
macro_rules! Depcrate_frame_utildebug_flags {
() => {
// Module: crate::frame::util
// Provides: {"debug_flags"}
// Dependencies: {}
pub (super) fn debug_flags < 'a , 'f : 'a > (fmt : & 'a mut fmt :: Formatter < 'f > , bits : u8 ,) -> DebugFlags < 'a , 'f > { let result = write ! (fmt , "({:#x}" , bits) ; DebugFlags { fmt , result , started : false , } }
};
}

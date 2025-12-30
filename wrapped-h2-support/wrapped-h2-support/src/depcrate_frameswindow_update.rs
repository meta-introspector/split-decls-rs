// Generated macro for window_update (function)
macro_rules! Depcrate_frameswindow_update {
() => {
// Module: crate::frames
// Provides: {"window_update"}
// Dependencies: {}
pub fn window_update < T > (id : T , sz : u32) -> frame :: WindowUpdate where T : Into < StreamId > , { frame :: WindowUpdate :: new (id . into () , sz) }
};
}

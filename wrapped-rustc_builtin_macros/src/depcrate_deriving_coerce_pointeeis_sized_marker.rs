// Generated macro for is_sized_marker (function)
macro_rules! Depcrate_deriving_coerce_pointeeis_sized_marker {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"is_sized_marker"}
// Dependencies: {}
fn is_sized_marker (path : & ast :: Path) -> bool { const CORE_UNSIZE : [Symbol ; 3] = [sym :: core , sym :: marker , sym :: Sized] ; const STD_UNSIZE : [Symbol ; 3] = [sym :: std , sym :: marker , sym :: Sized] ; if path . segments . len () == 4 && path . is_global () { path_segment_is_exact_match (& path . segments [1 ..] , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments [1 ..] , & STD_UNSIZE) } else if path . segments . len () == 3 { path_segment_is_exact_match (& path . segments , & CORE_UNSIZE) || path_segment_is_exact_match (& path . segments , & STD_UNSIZE) } else { * path == sym :: Sized } }
};
}

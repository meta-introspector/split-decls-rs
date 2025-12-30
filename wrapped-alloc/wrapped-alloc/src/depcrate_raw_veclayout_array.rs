// Generated macro for layout_array (function)
macro_rules! Depcrate_raw_veclayout_array {
() => {
// Module: crate::raw_vec
// Provides: {"layout_array"}
// Dependencies: {}
# [inline] fn layout_array (cap : usize , elem_layout : Layout) -> Result < Layout , TryReserveError > { elem_layout . repeat (cap) . map (| (layout , _pad) | layout) . map_err (| _ | CapacityOverflow . into ()) }
};
}

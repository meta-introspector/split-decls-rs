// Generated macro for layout_sum (function)
macro_rules! Depcrate_arenalayout_sum {
() => {
// Module: crate::arena
// Provides: {"layout_sum"}
// Dependencies: {}
# [doc = " A sum of layout size and align mask."] # [inline (always)] fn layout_sum (layout : & Layout) -> usize { layout . size () + (layout . align () - 1) }
};
}

// Generated macro for rc_inner_layout_for_value_layout (function)
macro_rules! Depcrate_rcrc_inner_layout_for_value_layout {
() => {
// Module: crate::rc
// Provides: {"rc_inner_layout_for_value_layout"}
// Dependencies: {}
# [doc = " Calculate layout for `RcInner<T>` using the inner value's layout"] fn rc_inner_layout_for_value_layout (layout : Layout) -> Layout { Layout :: new :: < RcInner < () > > () . extend (layout) . unwrap () . 0 . pad_to_align () }
};
}

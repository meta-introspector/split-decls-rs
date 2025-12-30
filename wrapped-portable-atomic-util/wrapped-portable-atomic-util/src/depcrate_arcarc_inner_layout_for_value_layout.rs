// Generated macro for arc_inner_layout_for_value_layout (function)
macro_rules! Depcrate_arcarc_inner_layout_for_value_layout {
() => {
// Module: crate::arc
// Provides: {"arc_inner_layout_for_value_layout"}
// Dependencies: {}
# [doc = " Calculate layout for `ArcInner<T>` using the inner value's layout"] fn arc_inner_layout_for_value_layout (layout : Layout) -> Layout { pad_to_align (extend_layout (Layout :: new :: < ArcInner < () > > () , layout) . unwrap () . 0) }
};
}

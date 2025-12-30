// Generated macro for arcinner_layout_for_value_layout (function)
macro_rules! Depcrate_syncarcinner_layout_for_value_layout {
() => {
// Module: crate::sync
// Provides: {"arcinner_layout_for_value_layout"}
// Dependencies: {}
# [doc = " Calculate layout for `ArcInner<T>` using the inner value's layout"] fn arcinner_layout_for_value_layout (layout : Layout) -> Layout { Layout :: new :: < ArcInner < () > > () . extend (layout) . unwrap () . 0 . pad_to_align () }
};
}

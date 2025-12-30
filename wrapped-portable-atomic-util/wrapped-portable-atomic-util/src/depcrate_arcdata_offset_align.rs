// Generated macro for data_offset_align (function)
macro_rules! Depcrate_arcdata_offset_align {
() => {
// Module: crate::arc
// Provides: {"data_offset_align"}
// Dependencies: {}
# [inline] fn data_offset_align (align : usize) -> usize { let layout = Layout :: new :: < ArcInner < () > > () ; layout . size () + padding_needed_for (layout , align) }
};
}

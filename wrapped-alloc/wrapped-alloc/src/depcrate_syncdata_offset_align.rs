// Generated macro for data_offset_align (function)
macro_rules! Depcrate_syncdata_offset_align {
() => {
// Module: crate::sync
// Provides: {"data_offset_align"}
// Dependencies: {}
# [inline] fn data_offset_align (align : usize) -> usize { let layout = Layout :: new :: < ArcInner < () > > () ; layout . size () + layout . padding_needed_for (align) }
};
}

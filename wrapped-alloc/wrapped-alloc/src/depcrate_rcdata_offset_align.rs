// Generated macro for data_offset_align (function)
macro_rules! Depcrate_rcdata_offset_align {
() => {
// Module: crate::rc
// Provides: {"data_offset_align"}
// Dependencies: {}
# [inline] fn data_offset_align (align : usize) -> usize { let layout = Layout :: new :: < RcInner < () > > () ; layout . size () + layout . padding_needed_for (align) }
};
}

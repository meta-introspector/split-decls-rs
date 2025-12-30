// Generated macro for data_offset (function)
macro_rules! Depcrate_arcdata_offset {
() => {
// Module: crate::arc
// Provides: {"data_offset"}
// Dependencies: {}
# [doc = " Gets the offset within an `ArcInner` for the payload behind a pointer."] fn data_offset < T : ? Sized > (ptr : & T) -> usize { data_offset_align (mem :: align_of_val :: < T > (ptr)) }
};
}

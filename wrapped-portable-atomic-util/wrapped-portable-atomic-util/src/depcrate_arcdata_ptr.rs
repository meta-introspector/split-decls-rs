// Generated macro for data_ptr (function)
macro_rules! Depcrate_arcdata_ptr {
() => {
// Module: crate::arc
// Provides: {"data_ptr"}
// Dependencies: {}
# [doc = " Gets the pointer to data within the given an `ArcInner`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `arc` must uphold the safety requirements for `.byte_add(data_offset)`."] # [doc = " This is automatically satisfied if it is a pointer to a valid `ArcInner`."] unsafe fn data_ptr < T : ? Sized > (arc : * mut ArcInner < T > , data : & T) -> * mut T { unsafe { let offset = data_offset :: < T > (data) ; strict :: byte_add (arc , offset) as * mut T } }
};
}

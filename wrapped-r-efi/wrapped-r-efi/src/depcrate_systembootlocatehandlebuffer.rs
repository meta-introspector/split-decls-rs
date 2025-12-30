// Generated macro for BootLocateHandleBuffer (type)
macro_rules! Depcrate_systemBootLocateHandleBuffer {
() => {
// Module: crate::system
// Provides: {"BootLocateHandleBuffer"}
// Dependencies: {}
pub type BootLocateHandleBuffer = eficall ! { fn (LocateSearchType , * mut crate :: base :: Guid , * mut core :: ffi :: c_void , * mut usize , * mut * mut crate :: base :: Handle ,) -> crate :: base :: Status } ;
};
}

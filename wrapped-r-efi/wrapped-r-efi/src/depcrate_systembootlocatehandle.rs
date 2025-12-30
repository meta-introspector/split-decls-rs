// Generated macro for BootLocateHandle (type)
macro_rules! Depcrate_systemBootLocateHandle {
() => {
// Module: crate::system
// Provides: {"BootLocateHandle"}
// Dependencies: {}
pub type BootLocateHandle = eficall ! { fn (LocateSearchType , * mut crate :: base :: Guid , * mut core :: ffi :: c_void , * mut usize , * mut crate :: base :: Handle ,) -> crate :: base :: Status } ;
};
}

// Generated macro for BootRegisterProtocolNotify (type)
macro_rules! Depcrate_systemBootRegisterProtocolNotify {
() => {
// Module: crate::system
// Provides: {"BootRegisterProtocolNotify"}
// Dependencies: {}
pub type BootRegisterProtocolNotify = eficall ! { fn (* mut crate :: base :: Guid , crate :: base :: Event , * mut * mut core :: ffi :: c_void ,) -> crate :: base :: Status } ;
};
}

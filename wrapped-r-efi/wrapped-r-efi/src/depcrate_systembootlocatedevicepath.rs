// Generated macro for BootLocateDevicePath (type)
macro_rules! Depcrate_systemBootLocateDevicePath {
() => {
// Module: crate::system
// Provides: {"BootLocateDevicePath"}
// Dependencies: {}
pub type BootLocateDevicePath = eficall ! { fn (* mut crate :: base :: Guid , * mut * mut crate :: protocols :: device_path :: Protocol , * mut crate :: base :: Handle ,) -> crate :: base :: Status } ;
};
}

// Generated macro for BootLoadImage (type)
macro_rules! Depcrate_systemBootLoadImage {
() => {
// Module: crate::system
// Provides: {"BootLoadImage"}
// Dependencies: {}
pub type BootLoadImage = eficall ! { fn (crate :: base :: Boolean , crate :: base :: Handle , * mut crate :: protocols :: device_path :: Protocol , * mut core :: ffi :: c_void , usize , * mut crate :: base :: Handle ,) -> crate :: base :: Status } ;
};
}

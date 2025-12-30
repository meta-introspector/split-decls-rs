// Generated macro for BootCreateEventEx (type)
macro_rules! Depcrate_systemBootCreateEventEx {
() => {
// Module: crate::system
// Provides: {"BootCreateEventEx"}
// Dependencies: {}
pub type BootCreateEventEx = eficall ! { fn (u32 , crate :: base :: Tpl , Option < EventNotify >, * const core :: ffi :: c_void , * const crate :: base :: Guid , * mut crate :: base :: Event ,) -> crate :: base :: Status } ;
};
}

// Generated macro for BootCreateEvent (type)
macro_rules! Depcrate_systemBootCreateEvent {
() => {
// Module: crate::system
// Provides: {"BootCreateEvent"}
// Dependencies: {}
pub type BootCreateEvent = eficall ! { fn (u32 , crate :: base :: Tpl , Option < EventNotify >, * mut core :: ffi :: c_void , * mut crate :: base :: Event ,) -> crate :: base :: Status } ;
};
}

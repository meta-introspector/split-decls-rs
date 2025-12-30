// Generated macro for BootAllocatePool (type)
macro_rules! Depcrate_systemBootAllocatePool {
() => {
// Module: crate::system
// Provides: {"BootAllocatePool"}
// Dependencies: {}
pub type BootAllocatePool = eficall ! { fn (MemoryType , usize , * mut * mut core :: ffi :: c_void ,) -> crate :: base :: Status } ;
};
}

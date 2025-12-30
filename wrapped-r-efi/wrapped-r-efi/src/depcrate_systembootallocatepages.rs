// Generated macro for BootAllocatePages (type)
macro_rules! Depcrate_systemBootAllocatePages {
() => {
// Module: crate::system
// Provides: {"BootAllocatePages"}
// Dependencies: {}
pub type BootAllocatePages = eficall ! { fn (AllocateType , MemoryType , usize , * mut crate :: base :: PhysicalAddress ,) -> crate :: base :: Status } ;
};
}

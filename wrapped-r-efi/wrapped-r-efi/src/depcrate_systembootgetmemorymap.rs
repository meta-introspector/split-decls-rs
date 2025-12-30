// Generated macro for BootGetMemoryMap (type)
macro_rules! Depcrate_systemBootGetMemoryMap {
() => {
// Module: crate::system
// Provides: {"BootGetMemoryMap"}
// Dependencies: {}
pub type BootGetMemoryMap = eficall ! { fn (* mut usize , * mut MemoryDescriptor , * mut usize , * mut usize , * mut u32 ,) -> crate :: base :: Status } ;
};
}

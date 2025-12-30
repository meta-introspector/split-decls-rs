// Generated macro for impl_815 (impl)
macro_rules! Depcrate_machineimpl_815 {
() => {
// Module: crate::machine
// Provides: {"impl_815"}
// Dependencies: {}
impl MiriMemoryKind { # [doc = " Whether we have a useful allocation span for an allocation of this kind."] fn should_save_allocation_span (self) -> bool { use self :: MiriMemoryKind :: * ; match self { Rust | Miri | C | WinHeap | WinLocal | Mmap => true , Machine | Global | ExternStatic | Tls | Runtime => false , } } }
};
}

// Generated macro for impl_814 (impl)
macro_rules! Depcrate_machineimpl_814 {
() => {
// Module: crate::machine
// Provides: {"impl_814"}
// Dependencies: {}
impl MayLeak for MiriMemoryKind { # [inline (always)] fn may_leak (self) -> bool { use self :: MiriMemoryKind :: * ; match self { Rust | Miri | C | WinHeap | WinLocal | Runtime => false , Machine | Global | ExternStatic | Tls | Mmap => true , } } }
};
}

// Generated macro for impl_979 (impl)
macro_rules! Depcrate_machineimpl_979 {
() => {
// Module: crate::machine
// Provides: {"impl_979"}
// Dependencies: {}
impl MayLeak for MiriMemoryKind { # [inline (always)] fn may_leak (self) -> bool { use self :: MiriMemoryKind :: * ; match self { Rust | Miri | C | WinHeap | WinLocal | Runtime => false , Machine | Global | ExternStatic | Tls | Mmap => true , } } }
};
}

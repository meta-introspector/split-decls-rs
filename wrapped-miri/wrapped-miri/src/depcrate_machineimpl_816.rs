// Generated macro for impl_816 (impl)
macro_rules! Depcrate_machineimpl_816 {
() => {
// Module: crate::machine
// Provides: {"impl_816"}
// Dependencies: {}
impl fmt :: Display for MiriMemoryKind { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: MiriMemoryKind :: * ; match self { Rust => write ! (f , "Rust heap") , Miri => write ! (f , "Miri bare-metal heap") , C => write ! (f , "C heap") , WinHeap => write ! (f , "Windows heap") , WinLocal => write ! (f , "Windows local memory") , Machine => write ! (f , "machine-managed memory") , Runtime => write ! (f , "language runtime memory") , Global => write ! (f , "global (static or const)") , ExternStatic => write ! (f , "extern static") , Tls => write ! (f , "thread-local static") , Mmap => write ! (f , "mmap") , } } }
};
}

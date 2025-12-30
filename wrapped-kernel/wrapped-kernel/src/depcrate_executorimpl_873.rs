// Generated macro for impl_873 (impl)
macro_rules! Depcrate_executorimpl_873 {
() => {
// Module: crate::executor
// Provides: {"impl_873"}
// Dependencies: {}
impl TaskNotify { pub const fn new () -> Self { Self { futex : AtomicU32 :: new (0) , } } pub fn wait (& self , timeout : Option < u64 >) { let _ = futex_wait_and_set (& self . futex , 0 , timeout , Flags :: RELATIVE , 0) ; } }
};
}

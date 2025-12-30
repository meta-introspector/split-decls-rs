// Generated macro for BorrowMustNotOutliveMutexTest (const)
macro_rules! Depcrate_mutexBorrowMustNotOutliveMutexTest {
() => {
// Module: crate::mutex
// Provides: {"BorrowMustNotOutliveMutexTest"}
// Dependencies: {}
# [doc = " ``` compile_fail"] # [doc = " fn bad(cs: critical_section::CriticalSection) -> &u32 {"] # [doc = "     let x = critical_section::Mutex::new(42u32);"] # [doc = "     x.borrow(cs)"] # [doc = " }"] # [doc = " ```"] # [cfg (doctest)] const BorrowMustNotOutliveMutexTest : () = () ;
};
}

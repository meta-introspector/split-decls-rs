// Generated macro for impl_1261 (impl)
macro_rules! Depcrate_test_runner_runnerimpl_1261 {
() => {
// Module: crate::test_runner::runner
// Provides: {"impl_1261"}
// Dependencies: {}
impl fmt :: Display for TestRunner { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "\tsuccesses: {}\n\
             \tlocal rejects: {}\n" , self . successes , self . local_rejects) ? ; for (whence , count) in & self . local_reject_detail { writeln ! (f , "\t\t{} times at {}" , count , whence) ? ; } writeln ! (f , "\tglobal rejects: {}" , self . global_rejects) ? ; for (whence , count) in & self . global_reject_detail { writeln ! (f , "\t\t{} times at {}" , count , whence) ? ; } Ok (()) } }
};
}

// Generated macro for ebr_panic_oom (function)
macro_rules! Depcrate_oomebr_panic_oom {
() => {
// Module: crate::oom
// Provides: {"ebr_panic_oom"}
// Dependencies: {}
fn ebr_panic_oom (repeat : usize) { for _ in 0 .. repeat { let _result : Result < () , Box < dyn Any + Send > > = test_oom (| | { Guard :: new () . accelerate () ; let r = Shared :: new (R :: new (& INST_CNT , false)) ; assert_ne ! (INST_CNT . load (Relaxed) , 0) ; drop (r) ; }) ; } }
};
}

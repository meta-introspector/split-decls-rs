// Generated macro for impl_303 (impl)
macro_rules! Depcrate_sleep_countersimpl_303 {
() => {
// Module: crate::sleep::counters
// Provides: {"impl_303"}
// Dependencies: {}
impl std :: fmt :: Debug for Counters { fn fmt (& self , fmt : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let word = format ! ("{:016x}" , self . word) ; fmt . debug_struct ("Counters") . field ("word" , & word) . field ("jobs" , & self . jobs_counter () . 0) . field ("inactive" , & self . inactive_threads ()) . field ("sleeping" , & self . sleeping_threads ()) . finish () } }
};
}

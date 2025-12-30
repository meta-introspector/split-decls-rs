// Generated macro for impl_17 (impl)
macro_rules! Depcrate_poolimpl_17 {
() => {
// Module: crate::pool
// Provides: {"impl_17"}
// Dependencies: {}
impl WorkerPool { # [doc = " Executes `f` in a web worker."] # [doc = ""] # [doc = " This pool manages a set of web workers to draw from, and `f` will be"] # [doc = " spawned quickly into one if the worker is idle. If no idle workers are"] # [doc = " available then a new web worker will be spawned."] # [doc = ""] # [doc = " Once `f` returns the worker assigned to `f` is automatically reclaimed"] # [doc = " by this `WorkerPool`. This method provides no method of learning when"] # [doc = " `f` completes, and for that you'll need to use `run_notify`."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " If an error happens while spawning a web worker or sending a message to"] # [doc = " a web worker, that error is returned."] pub fn run (& self , f : impl FnOnce () + Send + 'static) -> Result < () , JsValue > { let worker = self . execute (f) ? ; self . reclaim_on_message (worker) ; Ok (()) } }
};
}

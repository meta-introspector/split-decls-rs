// Generated macro for WakerInterest (enum)
macro_rules! Depcrate_waker_queueWakerInterest {
() => {
// Module: crate::waker_queue
// Provides: {"WakerInterest"}
// Dependencies: {}
# [doc = " Types of interests we would look into when `Accept`'s `Poll` is waked up by waker."] # [doc = ""] # [doc = " These interests should not be confused with `mio::Interest` and mostly not I/O related"] pub (crate) enum WakerInterest { # [doc = " `WorkerAvailable` is an interest from `Worker` notifying `Accept` there is a worker"] # [doc = " available and can accept new tasks."] WorkerAvailable (usize) , # [doc = " `Pause`, `Resume`, `Stop` Interest are from `ServerBuilder` future. It listens to"] # [doc = " `ServerCommand` and notify `Accept` to do exactly these tasks."] Pause , Resume , Stop , # [doc = " `Worker` is an interest that is triggered after a worker faults. This is determined by"] # [doc = " trying to send work to it. `Accept` would be waked up and add the new `WorkerHandleAccept`."] Worker (WorkerHandleAccept) , }
};
}

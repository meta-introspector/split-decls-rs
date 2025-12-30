// Generated macro for AsyncTask (struct)
macro_rules! Depcrate_executor_taskAsyncTask {
() => {
// Module: crate::executor::task
// Provides: {"AsyncTask"}
// Dependencies: {}
pub (crate) struct AsyncTask { id : AsyncTaskId , future : Pin < Box < dyn Future < Output = () > + Send > > , }
};
}

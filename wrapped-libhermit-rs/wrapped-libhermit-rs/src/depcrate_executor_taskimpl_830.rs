// Generated macro for impl_830 (impl)
macro_rules! Depcrate_executor_taskimpl_830 {
() => {
// Module: crate::executor::task
// Provides: {"impl_830"}
// Dependencies: {}
impl AsyncTask { pub fn new (future : impl Future < Output = () > + Send + 'static) -> AsyncTask { AsyncTask { id : AsyncTaskId :: new () , future : Box :: pin (future) , } } }
};
}

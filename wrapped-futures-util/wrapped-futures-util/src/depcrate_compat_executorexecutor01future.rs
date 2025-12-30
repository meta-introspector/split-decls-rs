// Generated macro for Executor01Future (type)
macro_rules! Depcrate_compat_executorExecutor01Future {
() => {
// Module: crate::compat::executor
// Provides: {"Executor01Future"}
// Dependencies: {}
# [doc = " A future that can run on a futures 0.1"] # [doc = " [`Executor`](futures_01::future::Executor)."] pub type Executor01Future = Compat < UnitError < FutureObj < 'static , () > > > ;
};
}

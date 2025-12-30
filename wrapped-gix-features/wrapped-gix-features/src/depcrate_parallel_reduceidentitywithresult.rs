// Generated macro for IdentityWithResult (struct)
macro_rules! Depcrate_parallel_reduceIdentityWithResult {
() => {
// Module: crate::parallel::reduce
// Provides: {"IdentityWithResult"}
// Dependencies: {}
# [doc = " An identity reducer for those who want to use [`Stepwise`] or [`in_parallel()`][crate::parallel::in_parallel()]"] # [doc = " without the use of non-threaded reduction of products created in threads."] pub struct IdentityWithResult < Input , Error > { _input : PhantomData < Input > , _error : PhantomData < Error > , }
};
}

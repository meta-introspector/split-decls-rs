macro_rules! IdentityWithResult {
    () => {
        # [doc = " An identity reducer for those who want to use [`Stepwise`] or [`in_parallel()`][crate::parallel::in_parallel()]"] # [doc = " without the use of non-threaded reduction of products created in threads."] pub struct IdentityWithResult < Input , Error > { _input : PhantomData < Input > , _error : PhantomData < Error > , }
    };
}

IdentityWithResult!()
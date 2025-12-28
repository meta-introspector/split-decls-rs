macro_rules! LocalFutureObj {
    () => {
        # [doc = " A custom trait object for polling futures, roughly akin to"] # [doc = " `Box<dyn Future<Output = T> + 'a>`."] # [doc = ""] # [doc = " This custom trait object was introduced as currently it is not possible to"] # [doc = " take `dyn Trait` by value and `Box<dyn Trait>` is not available in no_std"] # [doc = " contexts."] pub struct LocalFutureObj < 'a , T > { future : * mut (dyn Future < Output = T > + 'static) , drop_fn : unsafe fn (* mut (dyn Future < Output = T > + 'static)) , _marker : PhantomData < & 'a () > , }
    };
}

LocalFutureObj!();
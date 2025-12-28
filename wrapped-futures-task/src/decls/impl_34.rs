macro_rules! deps {
    () => {
        FutureObj!();
        UnsafeFutureObj!();
        LocalFutureObj!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a , T > LocalFutureObj < 'a , T > { # [doc = " Create a `LocalFutureObj` from a custom trait object representation."] # [inline] pub fn new < F : UnsafeFutureObj < 'a , T > + 'a > (f : F) -> Self { Self { future : unsafe { remove_future_lifetime (f . into_raw ()) } , drop_fn : unsafe { remove_drop_lifetime (F :: drop) } , _marker : PhantomData , } } # [doc = " Converts the `LocalFutureObj` into a `FutureObj`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " To make this operation safe one has to ensure that the `UnsafeFutureObj`"] # [doc = " instance from which this `LocalFutureObj` was created actually"] # [doc = " implements `Send`."] # [inline] pub unsafe fn into_future_obj (self) -> FutureObj < 'a , T > { FutureObj (self) } }
    };
}

impl_34!()
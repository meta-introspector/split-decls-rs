macro_rules! deps {
    () => {
        IntoConcurrentStream!();
        ConcurrentStream!();
    };
}

macro_rules! FromConcurrentStream {
    () => {
        deps!();
        # [doc = " Conversion from a [`ConcurrentStream`]"] # [allow (async_fn_in_trait)] pub trait FromConcurrentStream < A > : Sized { # [doc = " Creates a value from a concurrent iterator."] async fn from_concurrent_stream < T > (iter : T) -> Self where T : IntoConcurrentStream < Item = A > ; }
    };
}

FromConcurrentStream!();
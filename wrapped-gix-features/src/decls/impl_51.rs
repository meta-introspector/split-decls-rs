macro_rules! deps {
    () => {
        EagerIterIf!();
        EagerIter!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < I > EagerIterIf < I > where I : Iterator + Send + 'static , < I as Iterator > :: Item : Send , { # [doc = " Return a new `EagerIterIf` if `condition()` returns true."] # [doc = ""] # [doc = " For all other parameters, please see [`EagerIter::new()`]."] pub fn new (condition : impl FnOnce () -> bool , iter : I , chunk_size : usize , chunks_in_flight : usize) -> Self { if condition () { EagerIterIf :: Eager (EagerIter :: new (iter , chunk_size , chunks_in_flight)) } else { EagerIterIf :: OnDemand (iter) } } }
    };
}

impl_51!()
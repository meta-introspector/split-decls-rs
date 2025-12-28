macro_rules! deps {
    () => {
        EnumerateFuture!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < FutT , T > EnumerateFuture < FutT , T > where FutT : Future < Output = T > , { fn new (fut_t : FutT , count : usize) -> Self { Self { done : false , fut_t , count , } } }
    };
}

impl_132!();
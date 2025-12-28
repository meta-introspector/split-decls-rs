macro_rules! deps {
    () => {
        MapFuture!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < F , FutT , T , FutB , B > MapFuture < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = B > , { fn new (f : F , fut_t : FutT) -> Self { Self { done : false , f , fut_t : Some (fut_t) , fut_b : None , } } }
    };
}

impl_175!()
macro_rules! deps {
    () => {
        TryForEachFut!();
        Try!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < F , FutT , T , FutB , B > TryForEachFut < F , FutT , T , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { fn new (f : F , fut_t : FutT , count : Arc < AtomicUsize >) -> Self { Self { done : false , count , f , fut_t : Some (fut_t) , fut_b : None , } } }
    };
}

impl_189!();
macro_rules! deps {
    () => {
        ForEachFut!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < F , FutT , T , FutB > ForEachFut < F , FutT , T , FutB > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = () > , { fn new (f : F , fut_t : FutT , count : Arc < AtomicUsize >) -> Self { Self { done : false , count , f , fut_t : Some (fut_t) , fut_b : None , } } }
    };
}

impl_140!()
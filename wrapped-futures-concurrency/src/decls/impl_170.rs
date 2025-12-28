macro_rules! deps {
    () => {
        Map!();
        ConcurrentStream!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < CS , F , FutT , T , FutB , B > Map < CS , F , FutT , T , FutB , B > where CS : ConcurrentStream < Item = T , Future = FutT > , F : Fn (T) -> FutB , F : Clone , FutT : Future < Output = T > , FutB : Future < Output = B > , { pub (crate) fn new (inner : CS , f : F) -> Self { Self { inner , f , _phantom : PhantomData , } } }
    };
}

impl_170!()
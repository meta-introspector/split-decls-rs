macro_rules! deps {
    () => {
        ConcurrentStream!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " Convert items from one type into another"] # [derive (Debug)] pub struct Map < CS , F , FutT , T , FutB , B > where CS : ConcurrentStream < Item = T , Future = FutT > , F : Fn (T) -> FutB , F : Clone , FutT : Future < Output = T > , FutB : Future < Output = B > , { inner : CS , f : F , _phantom : PhantomData < (FutT , T , FutB , B) > , }
    };
}

Map!()
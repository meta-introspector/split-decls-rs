macro_rules! deps {
    () => {
        MapFuture!();
        Consumer!();
    };
}

macro_rules! MapConsumer {
    () => {
        deps!();
        # [pin_project] pub struct MapConsumer < C , F , FutT , T , FutB , B > where FutT : Future < Output = T > , C : Consumer < B , MapFuture < F , FutT , T , FutB , B > > , F : Fn (T) -> FutB , F : Clone , FutB : Future < Output = B > , { # [pin] inner : C , f : F , _phantom : PhantomData < (FutT , T , FutB , B) > , }
    };
}

MapConsumer!();
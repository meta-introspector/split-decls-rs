macro_rules! deps {
    () => {
        TryForEachFut!();
        Try!();
    };
}

macro_rules! TryForEachConsumer {
    () => {
        deps!();
        # [pin_project] pub (crate) struct TryForEachConsumer < FutT , T , F , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { count : Arc < AtomicUsize > , # [pin] group : FuturesUnordered < TryForEachFut < F , FutT , T , FutB , B > > , limit : usize , residual : Option < B :: Residual > , f : F , _phantom : PhantomData < (T , FutB) > , }
    };
}

TryForEachConsumer!()
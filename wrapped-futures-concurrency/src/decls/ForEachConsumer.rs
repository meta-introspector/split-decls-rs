macro_rules! deps {
    () => {
        ForEachFut!();
    };
}

macro_rules! ForEachConsumer {
    () => {
        deps!();
        # [pin_project] pub (crate) struct ForEachConsumer < FutT , T , F , FutB > where FutT : Future < Output = T > , F : Fn (T) -> FutB , FutB : Future < Output = () > , { count : Arc < AtomicUsize > , # [pin] group : FuturesUnordered < ForEachFut < F , FutT , T , FutB > > , limit : usize , f : F , _phantom : PhantomData < (T , FutB) > , }
    };
}

ForEachConsumer!();
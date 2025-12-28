macro_rules! deps {
    () => {
        TryForEachConsumer!();
        Try!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < FutT , T , F , FutB , B > TryForEachConsumer < FutT , T , F , FutB , B > where FutT : Future < Output = T > , F : Clone + Fn (T) -> FutB , FutB : Future < Output = B > , B : Try < Output = () > , { pub (crate) fn new (limit : Option < NonZeroUsize > , f : F) -> Self { let limit = match limit { Some (n) => n . get () , None => usize :: MAX , } ; Self { limit , f , residual : None , count : Arc :: new (AtomicUsize :: new (0)) , group : FuturesUnordered :: new () , _phantom : PhantomData , } } }
    };
}

impl_186!()
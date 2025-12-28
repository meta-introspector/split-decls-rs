macro_rules! deps {
    () => {
        ForEachConsumer!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < A , T , F , B > ForEachConsumer < A , T , F , B > where A : Future < Output = T > , F : Fn (T) -> B , B : Future < Output = () > , { pub (crate) fn new (limit : Option < NonZeroUsize > , f : F) -> Self { let limit = match limit { Some (n) => n . get () , None => usize :: MAX , } ; Self { limit , f , _phantom : PhantomData , count : Arc :: new (AtomicUsize :: new (0)) , group : FuturesUnordered :: new () , } } }
    };
}

impl_137!();
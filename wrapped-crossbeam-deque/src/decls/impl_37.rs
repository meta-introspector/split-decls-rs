macro_rules! deps {
    () => {
        Injector!();
        Block!();
        Position!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < T > Default for Injector < T > { fn default () -> Self { let block = Box :: into_raw (Block :: < T > :: new ()) ; Self { head : CachePadded :: new (Position { block : AtomicPtr :: new (block) , index : AtomicUsize :: new (0) , }) , tail : CachePadded :: new (Position { block : AtomicPtr :: new (block) , index : AtomicUsize :: new (0) , }) , _marker : PhantomData , } } }
    };
}

impl_37!()
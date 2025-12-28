macro_rules! deps {
    () => {
        TryFold!();
    };
}

macro_rules! impl_897 {
    () => {
        deps!();
        impl < I , U , ID , F > TryFold < I , U , ID , F > { pub (super) fn new (base : I , identity : ID , fold_op : F) -> Self { TryFold { base , identity , fold_op , marker : PhantomData , } } }
    };
}

impl_897!()
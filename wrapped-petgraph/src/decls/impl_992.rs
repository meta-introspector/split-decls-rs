macro_rules! deps {
    () => {
        IdIterator!();
        IndexType!();
        NodeIdentifiers!();
    };
}

macro_rules! impl_992 {
    () => {
        deps!();
        impl < 'a , Ix : IndexType , S > NodeIdentifiers < 'a , Ix , S > { fn new (iter : IdIterator < 'a , S >) -> Self { Self { iter , ix : PhantomData , } } }
    };
}

impl_992!();
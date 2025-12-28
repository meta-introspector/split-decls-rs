macro_rules! deps {
    () => {
        DashSetVisitor!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < K , S > DashSetVisitor < K , S > where K : Eq + Hash , S : BuildHasher + Clone , { fn new () -> Self { DashSetVisitor { marker : PhantomData , } } }
    };
}

impl_97!()
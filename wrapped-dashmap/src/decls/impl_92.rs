macro_rules! deps {
    () => {
        DashMapVisitor!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < K , V , S > DashMapVisitor < K , V , S > where K : Eq + Hash , S : BuildHasher + Clone , { fn new () -> Self { DashMapVisitor { marker : PhantomData , } } }
    };
}

impl_92!()
macro_rules! deps {
    () => {
        DashMap!();
    };
}

macro_rules! DashMapVisitor {
    () => {
        deps!();
        pub struct DashMapVisitor < K , V , S > { marker : PhantomData < fn () -> DashMap < K , V , S > > , }
    };
}

DashMapVisitor!()
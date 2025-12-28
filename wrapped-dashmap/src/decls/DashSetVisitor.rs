macro_rules! deps {
    () => {
        DashSet!();
    };
}

macro_rules! DashSetVisitor {
    () => {
        deps!();
        pub struct DashSetVisitor < K , S > { marker : PhantomData < fn () -> DashSet < K , S > > , }
    };
}

DashSetVisitor!();
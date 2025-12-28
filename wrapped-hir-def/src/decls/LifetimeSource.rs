macro_rules! deps {
    () => {
        LifetimePtr!();
    };
}

macro_rules! LifetimeSource {
    () => {
        deps!();
        pub type LifetimeSource = InFile < LifetimePtr > ;
    };
}

LifetimeSource!();
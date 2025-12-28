macro_rules! deps {
    () => {
        Functions!();
        LazyResult!();
    };
}

macro_rules! LazyFunctions {
    () => {
        deps!();
        pub (crate) struct LazyFunctions < R : gimli :: Reader > (LazyResult < Functions < R > >) ;
    };
}

LazyFunctions!()
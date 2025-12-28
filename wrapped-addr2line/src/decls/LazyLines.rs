macro_rules! deps {
    () => {
        LazyResult!();
        Lines!();
    };
}

macro_rules! LazyLines {
    () => {
        deps!();
        pub (crate) struct LazyLines (LazyResult < Lines >) ;
    };
}

LazyLines!();
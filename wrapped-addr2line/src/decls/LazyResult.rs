macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! LazyResult {
    () => {
        deps!();
        type LazyResult < T > = OnceCell < Result < T , Error > > ;
    };
}

LazyResult!()
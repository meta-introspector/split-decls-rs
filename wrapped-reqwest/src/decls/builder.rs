macro_rules! deps {
    () => {
        Error!();
        Kind!();
        BoxError!();
    };
}

macro_rules! builder {
    () => {
        deps!();
        pub (crate) fn builder < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Builder , Some (e)) }
    };
}

builder!()
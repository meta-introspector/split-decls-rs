macro_rules! deps {
    () => {
        Error!();
        BoxError!();
        Kind!();
    };
}

macro_rules! body {
    () => {
        deps!();
        pub (crate) fn body < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Body , Some (e)) }
    };
}

body!()
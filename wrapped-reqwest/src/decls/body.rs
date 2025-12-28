macro_rules! deps {
    () => {
        Error!();
        Kind!();
        BoxError!();
    };
}

macro_rules! body {
    () => {
        deps!();
        pub (crate) fn body < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Body , Some (e)) }
    };
}

body!();
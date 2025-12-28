macro_rules! deps {
    () => {
        Error!();
        Kind!();
        BoxError!();
    };
}

macro_rules! request {
    () => {
        deps!();
        pub (crate) fn request < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Request , Some (e)) }
    };
}

request!()
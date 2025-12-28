macro_rules! deps {
    () => {
        BoxError!();
        Error!();
        Kind!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        pub (crate) fn decode < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Decode , Some (e)) }
    };
}

decode!();
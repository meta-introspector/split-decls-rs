macro_rules! deps {
    () => {
        Kind!();
        BoxError!();
        Error!();
    };
}

macro_rules! decode {
    () => {
        deps!();
        pub (crate) fn decode < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Decode , Some (e)) }
    };
}

decode!()
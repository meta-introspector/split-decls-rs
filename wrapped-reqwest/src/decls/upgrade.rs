macro_rules! deps {
    () => {
        BoxError!();
        Kind!();
        Error!();
    };
}

macro_rules! upgrade {
    () => {
        deps!();
        pub (crate) fn upgrade < E : Into < BoxError > > (e : E) -> Error { Error :: new (Kind :: Upgrade , Some (e)) }
    };
}

upgrade!()
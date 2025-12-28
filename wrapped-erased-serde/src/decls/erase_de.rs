macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! erase_de {
    () => {
        deps!();
        pub (crate) fn erase_de < E : serde :: de :: Error > (e : E) -> Error { serde :: de :: Error :: custom (e) }
    };
}

erase_de!()
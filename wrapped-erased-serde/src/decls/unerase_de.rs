macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! unerase_de {
    () => {
        deps!();
        pub (crate) fn unerase_de < E : serde :: de :: Error > (e : Error) -> E { e . as_serde_de_error () }
    };
}

unerase_de!();
macro_rules! deps {
    () => {
        FixupError!();
    };
}

macro_rules! FixupResult {
    () => {
        deps!();
        pub (crate) type FixupResult < T > = Result < T , FixupError > ;
    };
}

FixupResult!();
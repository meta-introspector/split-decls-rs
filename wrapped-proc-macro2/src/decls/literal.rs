macro_rules! deps {
    () => {
        Literal!();
        Cursor!();
        PResult!();
    };
}

macro_rules! literal {
    () => {
        deps!();
        pub (crate) fn literal (input : Cursor) -> PResult < Literal > { let rest = literal_nocapture (input) ? ; let end = input . len () - rest . len () ; Ok ((rest , Literal :: _new (input . rest [.. end] . to_string ()))) }
    };
}

literal!()
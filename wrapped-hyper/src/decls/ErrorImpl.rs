macro_rules! deps {
    () => {
        Cause!();
        Kind!();
    };
}

macro_rules! ErrorImpl {
    () => {
        deps!();
        struct ErrorImpl { kind : Kind , cause : Option < Cause > , }
    };
}

ErrorImpl!();
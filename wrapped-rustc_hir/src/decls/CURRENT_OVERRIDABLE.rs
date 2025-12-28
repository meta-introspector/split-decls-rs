macro_rules! deps {
    () => {
        RustcVersion!();
    };
}

macro_rules! CURRENT_OVERRIDABLE {
    () => {
        deps!();
        static CURRENT_OVERRIDABLE : OnceLock < RustcVersion > = OnceLock :: new () ;
    };
}

CURRENT_OVERRIDABLE!()
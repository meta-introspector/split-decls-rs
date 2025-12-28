macro_rules! deps {
    () => {
        UnparkToken!();
    };
}

macro_rules! DEFAULT_UNPARK_TOKEN {
    () => {
        deps!();
        # [doc = " A default unpark token to use."] pub const DEFAULT_UNPARK_TOKEN : UnparkToken = UnparkToken (0) ;
    };
}

DEFAULT_UNPARK_TOKEN!();
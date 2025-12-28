macro_rules! deps {
    () => {
        ErrorCode!();
    };
}

macro_rules! ErrorImpl {
    () => {
        deps!();
        struct ErrorImpl { code : ErrorCode , line : usize , column : usize , }
    };
}

ErrorImpl!()
macro_rules! deps {
    () => {
        Read!();
        ErrorCode!();
        Result!();
        Error!();
    };
}

macro_rules! error {
    () => {
        deps!();
        fn error < 'de , R , T > (read : & R , reason : ErrorCode) -> Result < T > where R : ? Sized + Read < 'de > , { let position = read . position () ; Err (Error :: syntax (reason , position . line , position . column)) }
    };
}

error!()
macro_rules! deps {
    () => {
        AutoCfg!();
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl error :: Error for Error { fn description (& self) -> & str { "AutoCfg error" } fn cause (& self) -> Option < & error :: Error > { match self . kind { ErrorKind :: Io (ref e) => Some (e) , ErrorKind :: Num (ref e) => Some (e) , ErrorKind :: Utf8 (ref e) => Some (e) , ErrorKind :: Process (_) | ErrorKind :: Other (_) => None , } } }
    };
}

impl_2!();
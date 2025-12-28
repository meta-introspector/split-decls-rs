macro_rules! deps {
    () => {
        ErrorCode!();
        Error!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl serde :: de :: StdError for Error { # [cfg (feature = "std")] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match & self . err . code { ErrorCode :: Io (err) => err . source () , _ => None , } } }
    };
}

impl_64!();
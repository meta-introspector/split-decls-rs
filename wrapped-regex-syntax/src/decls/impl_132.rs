macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl From < hir :: Error > for Error { fn from (err : hir :: Error) -> Error { Error :: Translate (err) } }
    };
}

impl_132!();
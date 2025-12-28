macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl From < glob :: PatternError > for Error { fn from (e : glob :: PatternError) -> Self { Error :: GlobPattern (e) } }
    };
}

impl_41!()
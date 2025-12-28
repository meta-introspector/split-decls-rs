macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl From < glob :: GlobError > for Error { fn from (e : glob :: GlobError) -> Self { Error :: Glob (e) } }
    };
}

impl_40!()
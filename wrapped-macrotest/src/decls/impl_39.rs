macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl From < toml :: de :: Error > for Error { fn from (e : toml :: de :: Error) -> Self { Error :: TomlDe (e) } }
    };
}

impl_39!()
macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl From < toml :: ser :: Error > for Error { fn from (e : toml :: ser :: Error) -> Self { Error :: TomlSer (e) } }
    };
}

impl_38!();
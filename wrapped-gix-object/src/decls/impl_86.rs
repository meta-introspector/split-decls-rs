macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl From < Error > for io :: Error { fn from (err : Error) -> Self { io :: Error :: other (err) } }
    };
}

impl_86!();
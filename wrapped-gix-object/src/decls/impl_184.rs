macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl From < Error > for io :: Error { fn from (other : Error) -> io :: Error { io :: Error :: other (other) } }
    };
}

impl_184!()
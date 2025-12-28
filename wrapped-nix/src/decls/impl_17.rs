macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl From < Errno > for io :: Error { fn from (err : Errno) -> Self { io :: Error :: from_raw_os_error (err as i32) } }
    };
}

impl_17!();
macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < Error > for std :: io :: Error { fn from (from : Error) -> Self { Self :: from_raw_os_error (from . code () . 0) } }
    };
}

impl_67!()
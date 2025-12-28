macro_rules! deps {
    () => {
        MZError!();
        StreamResult!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        # [cfg (not (feature = "rustc-dep-of-std"))] impl StreamResult { # [inline] pub const fn error (error : MZError) -> StreamResult { StreamResult { bytes_consumed : 0 , bytes_written : 0 , status : Err (error) , } } }
    };
}

impl_13!()
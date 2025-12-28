macro_rules! deps {
    () => {
        StreamResult!();
        MZError!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        # [cfg (not (feature = "rustc-dep-of-std"))] impl StreamResult { # [inline] pub const fn error (error : MZError) -> StreamResult { StreamResult { bytes_consumed : 0 , bytes_written : 0 , status : Err (error) , } } }
    };
}

impl_236!();
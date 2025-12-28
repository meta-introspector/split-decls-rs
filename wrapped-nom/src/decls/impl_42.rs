macro_rules! deps {
    () => {
        ErrorStr!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl From < u32 > for ErrorStr { fn from (i : u32) -> Self { ErrorStr (format ! ("custom error code: {}" , i)) } }
    };
}

impl_42!();
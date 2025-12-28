macro_rules! deps {
    () => {
        ErrorStr!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a > From < & 'a str > for ErrorStr { fn from (i : & 'a str) -> Self { ErrorStr (format ! ("custom error message: {}" , i)) } }
    };
}

impl_43!();
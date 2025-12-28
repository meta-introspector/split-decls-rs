macro_rules! deps {
    () => {
        Error!();
        ErrorImpl!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl DerefMut for Error { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { ErrorImpl :: error_mut (self . inner . by_mut ()) } } }
    };
}

impl_49!();
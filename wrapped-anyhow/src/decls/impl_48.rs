macro_rules! deps {
    () => {
        StdError!();
        ErrorImpl!();
        Error!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl Deref for Error { type Target = dyn StdError + Send + Sync + 'static ; fn deref (& self) -> & Self :: Target { unsafe { ErrorImpl :: error (self . inner . by_ref ()) } } }
    };
}

impl_48!()
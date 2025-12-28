macro_rules! deps {
    () => {
        Error!();
        StdError!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl AsRef < dyn StdError + Send + Sync > for Error { fn as_ref (& self) -> & (dyn StdError + Send + Sync + 'static) { & * * self } }
    };
}

impl_81!()
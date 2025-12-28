macro_rules! deps {
    () => {
        StdError!();
        Error!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        # [cfg (any (feature = "std" , not (anyhow_no_core_error)))] impl AsRef < dyn StdError > for Error { fn as_ref (& self) -> & (dyn StdError + 'static) { & * * self } }
    };
}

impl_82!();
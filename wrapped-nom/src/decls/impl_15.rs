macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I : ToOwned + ? Sized > Error < & I > { # [doc = " Converts `Error<&I>` into `Error<I::Owned>` by cloning."] pub fn cloned (self) -> Error < I :: Owned > { Error { input : self . input . to_owned () , code : self . code , } } }
    };
}

impl_15!();
macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < I : ToOwned + ? Sized > Error < & mut I > { # [doc = " Converts `Error<&mut I>` into `Error<I::Owned>` by cloning."] pub fn cloned (self) -> Error < I :: Owned > { Error { input : self . input . to_owned () , code : self . code , } } }
    };
}

impl_16!()
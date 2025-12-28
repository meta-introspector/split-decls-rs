macro_rules! deps {
    () => {
        Err!();
        ErrorKind!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Err < (& [u8] , ErrorKind) > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < (Vec < u8 > , ErrorKind) > { self . map_input (ToOwned :: to_owned) } }
    };
}

impl_151!()
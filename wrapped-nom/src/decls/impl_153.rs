macro_rules! deps {
    () => {
        Err!();
        Error!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Err < error :: Error < & [u8] > > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < error :: Error < Vec < u8 > > > { self . map_input (ToOwned :: to_owned) } }
    };
}

impl_153!();
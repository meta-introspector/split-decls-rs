macro_rules! deps {
    () => {
        Err!();
        Error!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Err < error :: Error < & str > > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < error :: Error < String > > { self . map_input (ToOwned :: to_owned) } }
    };
}

impl_154!()
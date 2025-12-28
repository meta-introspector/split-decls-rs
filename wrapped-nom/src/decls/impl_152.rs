macro_rules! deps {
    () => {
        Err!();
        ErrorKind!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Err < (& str , ErrorKind) > { # [doc = " Obtaining ownership"] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] pub fn to_owned (self) -> Err < (String , ErrorKind) > { self . map_input (ToOwned :: to_owned) } }
    };
}

impl_152!();
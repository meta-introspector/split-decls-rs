macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] impl From < Error < & [u8] > > for Error < crate :: lib :: std :: vec :: Vec < u8 > > { fn from (value : Error < & [u8] >) -> Self { Error { input : value . input . to_owned () , code : value . code , } } }
    };
}

impl_20!()
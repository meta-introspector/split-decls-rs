macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [cfg_attr (feature = "docsrs" , doc (cfg (feature = "alloc")))] impl From < Error < & str > > for Error < crate :: lib :: std :: string :: String > { fn from (value : Error < & str >) -> Self { Error { input : value . input . to_owned () , code : value . code , } } }
    };
}

impl_21!();
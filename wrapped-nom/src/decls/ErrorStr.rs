macro_rules! ErrorStr {
    () => {
        # [cfg (feature = "alloc")] # [derive (Debug , Clone , Eq , PartialEq)] pub struct ErrorStr (String) ;
    };
}

ErrorStr!();
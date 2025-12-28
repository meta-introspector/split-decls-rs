macro_rules! HashValue {
    () => {
        # [derive (Clone , Copy , Eq , PartialEq)] # [cfg_attr (feature = "zeroize" , derive (Zeroize))] struct HashValue (u16) ;
    };
}

HashValue!();
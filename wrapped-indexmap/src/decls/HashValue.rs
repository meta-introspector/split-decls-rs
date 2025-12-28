macro_rules! HashValue {
    () => {
        # [doc = " Hash value newtype. Not larger than usize, since anything larger"] # [doc = " isn't used for selecting position anyway."] # [derive (Clone , Copy , Debug , PartialEq)] struct HashValue (usize) ;
    };
}

HashValue!();
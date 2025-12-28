macro_rules! LocalKeyId {
    () => {
        # [derive (Eq , PartialEq , Hash , Copy , Clone)] struct LocalKeyId (usize) ;
    };
}

LocalKeyId!();
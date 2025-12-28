macro_rules! StaticKeyId {
    () => {
        # [derive (Eq , PartialEq , Hash , Copy , Clone)] pub (crate) struct StaticKeyId (usize) ;
    };
}

StaticKeyId!()
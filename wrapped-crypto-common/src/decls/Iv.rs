macro_rules! deps {
    () => {
        IvSizeUser!();
    };
}

macro_rules! Iv {
    () => {
        deps!();
        # [doc = " Initialization vector (nonce) used by [`IvSizeUser`] implementors."] pub type Iv < B > = Array < u8 , < B as IvSizeUser > :: IvSize > ;
    };
}

Iv!()
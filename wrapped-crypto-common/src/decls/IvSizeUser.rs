macro_rules! deps {
    () => {
        KeyIvInit!();
        InnerIvInit!();
    };
}

macro_rules! IvSizeUser {
    () => {
        deps!();
        # [doc = " Types which use initialization vector (nonce) for initialization."] # [doc = ""] # [doc = " Generally it's used indirectly via [`KeyIvInit`] or [`InnerIvInit`]."] pub trait IvSizeUser { # [doc = " Initialization vector size in bytes."] type IvSize : ArraySize ; # [doc = " Return IV size in bytes."] # [inline (always)] fn iv_size () -> usize { Self :: IvSize :: USIZE } }
    };
}

IvSizeUser!()
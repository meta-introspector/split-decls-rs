// Generated macro for Iv (type)
macro_rules! DepcrateIv {
() => {
// Module: crate
// Provides: {"Iv"}
// Dependencies: {}
# [doc = " Initialization vector (nonce) used by [`IvSizeUser`] implementors."] pub type Iv < B > = Array < u8 , < B as IvSizeUser > :: IvSize > ;
};
}

// Generated macro for Tag (type)
macro_rules! DepcrateTag {
() => {
// Module: crate
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " Tag: authentication code which ensures ciphertexts are authentic"] pub type Tag < A > = Array < u8 , < A as AeadCore > :: TagSize > ;
};
}

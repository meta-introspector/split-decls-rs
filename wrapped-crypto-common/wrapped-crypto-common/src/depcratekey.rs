// Generated macro for Key (type)
macro_rules! DepcrateKey {
() => {
// Module: crate
// Provides: {"Key"}
// Dependencies: {}
# [doc = " Key used by [`KeySizeUser`] implementors."] pub type Key < B > = Array < u8 , < B as KeySizeUser > :: KeySize > ;
};
}

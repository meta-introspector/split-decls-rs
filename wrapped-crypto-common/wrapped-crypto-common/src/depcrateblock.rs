// Generated macro for Block (type)
macro_rules! DepcrateBlock {
() => {
// Module: crate
// Provides: {"Block"}
// Dependencies: {}
# [doc = " Block on which [`BlockSizeUser`] implementors operate."] pub type Block < B > = Array < u8 , < B as BlockSizeUser > :: BlockSize > ;
};
}

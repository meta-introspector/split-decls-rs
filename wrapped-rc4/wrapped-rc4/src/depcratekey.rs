// Generated macro for Key (type)
macro_rules! DepcrateKey {
() => {
// Module: crate
// Provides: {"Key"}
// Dependencies: {}
# [doc = " RC4 key type (8–2048 bits/ 1-256 bytes)"] # [doc = ""] # [doc = " Implemented as an alias for [`Array`]."] pub type Key < KeySize > = Array < u8 , KeySize > ;
};
}

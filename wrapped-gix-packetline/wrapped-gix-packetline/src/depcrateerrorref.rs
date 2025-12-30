// Generated macro for ErrorRef (struct)
macro_rules! DepcrateErrorRef {
() => {
// Module: crate
// Provides: {"ErrorRef"}
// Dependencies: {}
# [doc = " A packet line representing an Error in a sideband channel."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct ErrorRef < 'a > (pub & 'a [u8]) ;
};
}

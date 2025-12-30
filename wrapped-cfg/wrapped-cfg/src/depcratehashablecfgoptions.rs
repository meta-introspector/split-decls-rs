// Generated macro for HashableCfgOptions (struct)
macro_rules! DepcrateHashableCfgOptions {
() => {
// Module: crate
// Provides: {"HashableCfgOptions"}
// Dependencies: {}
# [doc = " A `CfgOptions` that implements `Hash`, for the sake of hashing only."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct HashableCfgOptions { _enabled : Box < [CfgAtom] > , }
};
}

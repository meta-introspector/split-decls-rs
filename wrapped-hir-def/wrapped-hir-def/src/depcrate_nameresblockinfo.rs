// Generated macro for BlockInfo (struct)
macro_rules! Depcrate_nameresBlockInfo {
() => {
// Module: crate::nameres
// Provides: {"BlockInfo"}
// Dependencies: {}
# [doc = " For `DefMap`s computed for a block expression, this stores its location in the parent map."] # [derive (Debug , PartialEq , Eq , Clone , Copy)] struct BlockInfo { # [doc = " The `BlockId` this `DefMap` was created from."] block : BlockId , # [doc = " The containing module."] parent : BlockRelativeModuleId , }
};
}

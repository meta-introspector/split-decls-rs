// Generated macro for FdId (struct)
macro_rules! Depcrate_shims_filesFdId {
() => {
// Module: crate::shims::files
// Provides: {"FdId"}
// Dependencies: {}
# [doc = " A unique id for file descriptions. While we could use the address, considering that"] # [doc = " is definitely unique, the address would expose interpreter internal state when used"] # [doc = " for sorting things. So instead we generate a unique id per file description is the name"] # [doc = " for all `dup`licates and is never reused."] # [derive (Debug , Copy , Clone , Default , Eq , PartialEq , Ord , PartialOrd)] pub struct FdId (usize) ;
};
}

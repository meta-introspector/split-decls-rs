// Generated macro for LookupTableStatus (enum)
macro_rules! Depcrate_stateLookupTableStatus {
() => {
// Module: crate::state
// Provides: {"LookupTableStatus"}
// Dependencies: {}
# [doc = " Activation status of a lookup table"] # [derive (Debug , PartialEq , Eq , Clone)] pub enum LookupTableStatus { Activated , Deactivating { remaining_blocks : usize } , Deactivated , }
};
}

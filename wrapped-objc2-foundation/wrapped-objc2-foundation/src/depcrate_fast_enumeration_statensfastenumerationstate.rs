// Generated macro for NSFastEnumerationState (struct)
macro_rules! Depcrate_fast_enumeration_stateNSFastEnumerationState {
() => {
// Module: crate::fast_enumeration_state
// Provides: {"NSFastEnumerationState"}
// Dependencies: {}
# [repr (C)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct NSFastEnumerationState { pub state : c_ulong , pub itemsPtr : * mut * mut AnyObject , pub mutationsPtr : * mut c_ulong , pub extra : [c_ulong ; 5] , }
};
}

// Generated macro for TransitState (enum)
macro_rules! Depcrate_dirTransitState {
() => {
// Module: crate::dir
// Provides: {"TransitState"}
// Dependencies: {}
# [doc = ""] # [derive (Hash , Eq , PartialEq , Clone)] pub enum TransitState { # [doc = " Standard state."] Normal , # [doc = " Pause state when destination path exists."] Exists , # [doc = " Pause state when current process does not have the permission to access from or to"] # [doc = " path."] NoAccess , }
};
}

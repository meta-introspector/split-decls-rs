// Generated macro for Action (enum)
macro_rules! Depcrate_indexAction {
() => {
// Module: crate::index
// Provides: {"Action"}
// Dependencies: {}
# [doc = " What to do after a [ChangeRef] was passed ot the callback of [`index()`](crate::index())."] # [derive (Default , Clone , Copy , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Action { # [doc = " Continue the operation."] # [default] Continue , # [doc = " Stop the operation immediately."] # [doc = ""] # [doc = " This is useful if one just wants to determine if something changed or not."] Cancel , }
};
}

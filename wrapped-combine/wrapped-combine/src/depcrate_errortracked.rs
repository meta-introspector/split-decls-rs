// Generated macro for Tracked (struct)
macro_rules! Depcrate_errorTracked {
() => {
// Module: crate::error
// Provides: {"Tracked"}
// Dependencies: {}
# [doc = " Error wrapper which lets parsers track which parser in a sequence of sub-parsers has emitted"] # [doc = " the error. `Tracked::from` can be used to construct this and it should otherwise be"] # [doc = " ignored outside of combine."] # [derive (Clone , PartialEq , Debug , Copy)] pub struct Tracked < E > { # [doc = " The error returned"] pub error : E , # [doc (hidden)] pub offset : ErrorOffset , }
};
}

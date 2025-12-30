// Generated macro for Timeout (enum)
macro_rules! Depcrate_selectTimeout {
() => {
// Module: crate::select
// Provides: {"Timeout"}
// Dependencies: {}
# [doc = " Determines when a select operation should time out."] # [derive (Clone , Copy , Eq , PartialEq)] enum Timeout { # [doc = " No blocking."] Now , # [doc = " Block forever."] Never , # [doc = " Time out after the time instant."] At (Instant) , }
};
}

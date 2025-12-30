// Generated macro for macro_148 (macro)
macro_rules! Depcrate_limitedmacro_148 {
() => {
// Module: crate::limited
// Provides: {"macro_148"}
// Dependencies: {}
pin_project ! { # [doc = " A length limited body."] # [doc = ""] # [doc = " This body will return an error if more than the configured number"] # [doc = " of bytes are returned on polling the wrapped body."] # [derive (Clone , Copy , Debug)] pub struct Limited < B > { remaining : usize , # [pin] inner : B , } }
};
}

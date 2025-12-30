// Generated macro for unsupported (function)
macro_rules! Depcrate_stringunsupported {
() => {
// Module: crate::string
// Provides: {"unsupported"}
// Dependencies: {}
fn unsupported < T > (error : & 'static str) -> Result < T , Error > { Err (Error :: UnsupportedRegex (error)) }
};
}

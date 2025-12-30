// Generated macro for Result (type)
macro_rules! DepcrateResult {
() => {
// Module: crate
// Provides: {"Result"}
// Dependencies: {}
# [doc = " A Result of any parsing action."] # [doc = ""] # [doc = " If the input is invalid, an `Error` will be returned. Note that incomplete"] # [doc = " data is not considered invalid, and so will not return an error, but rather"] # [doc = " a `Ok(Status::Partial)`."] pub type Result < T > = result :: Result < Status < T > , Error > ;
};
}

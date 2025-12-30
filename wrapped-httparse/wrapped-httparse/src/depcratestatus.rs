// Generated macro for Status (enum)
macro_rules! DepcrateStatus {
() => {
// Module: crate
// Provides: {"Status"}
// Dependencies: {}
# [doc = " The result of a successful parse pass."] # [doc = ""] # [doc = " `Complete` is used when the buffer contained the complete value."] # [doc = " `Partial` is used when parsing did not reach the end of the expected value,"] # [doc = " but no invalid data was found."] # [derive (Copy , Clone , Eq , PartialEq , Debug)] pub enum Status < T > { # [doc = " The completed result."] Complete (T) , # [doc = " A partial result."] Partial }
};
}

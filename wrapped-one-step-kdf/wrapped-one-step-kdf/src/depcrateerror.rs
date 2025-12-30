// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " One-Step KDF errors."] # [derive (Clone , Copy , Debug , PartialEq)] pub enum Error { # [doc = " The length of the secret is zero."] NoSecret , # [doc = " The length of the output is zero."] NoOutput , # [doc = " The length of the output is too big."] CounterOverflow , }
};
}

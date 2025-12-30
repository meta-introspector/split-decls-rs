// Generated macro for check_disabled (function)
macro_rules! Depcratecheck_disabled {
() => {
// Module: crate
// Provides: {"check_disabled"}
// Dependencies: {}
# [doc = " Automates the `if is_disabled() { return error }` check and ensures"] # [doc = " we produce a consistent error message for it."] fn check_disabled () -> Result < () , Error > { if is_disabled () { return Err (Error :: new (ErrorKind :: Disabled , "the `cc` crate's functionality has been disabled by the `CC_FORCE_DISABLE` environment variable.")) ; } Ok (()) }
};
}

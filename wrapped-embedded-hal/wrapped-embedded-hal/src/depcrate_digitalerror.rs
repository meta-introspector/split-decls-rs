// Generated macro for Error (trait)
macro_rules! Depcrate_digitalError {
() => {
// Module: crate::digital
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error."] pub trait Error : core :: fmt :: Debug { # [doc = " Convert error to a generic error kind"] # [doc = ""] # [doc = " By using this method, errors freely defined by HAL implementations"] # [doc = " can be converted to a set of generic errors upon which generic"] # [doc = " code can act."] fn kind (& self) -> ErrorKind ; }
};
}

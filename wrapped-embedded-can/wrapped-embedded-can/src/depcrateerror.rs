// Generated macro for Error (trait)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " CAN error"] pub trait Error : core :: fmt :: Debug { # [doc = " Convert error to a generic CAN error kind"] # [doc = ""] # [doc = " By using this method, CAN errors freely defined by HAL implementations"] # [doc = " can be converted to a set of generic serial errors upon which generic"] # [doc = " code can act."] fn kind (& self) -> ErrorKind ; }
};
}

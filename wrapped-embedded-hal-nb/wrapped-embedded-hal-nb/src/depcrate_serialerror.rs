// Generated macro for Error (trait)
macro_rules! Depcrate_serialError {
() => {
// Module: crate::serial
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Serial error."] pub trait Error : core :: fmt :: Debug { # [doc = " Convert error to a generic serial error kind"] # [doc = ""] # [doc = " By using this method, serial errors freely defined by HAL implementations"] # [doc = " can be converted to a set of generic serial errors upon which generic"] # [doc = " code can act."] fn kind (& self) -> ErrorKind ; }
};
}

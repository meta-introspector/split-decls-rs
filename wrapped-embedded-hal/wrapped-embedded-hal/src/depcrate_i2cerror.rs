// Generated macro for Error (trait)
macro_rules! Depcrate_i2cError {
() => {
// Module: crate::i2c
// Provides: {"Error"}
// Dependencies: {}
# [doc = " I2C error."] pub trait Error : core :: fmt :: Debug { # [doc = " Convert error to a generic I2C error kind."] # [doc = ""] # [doc = " By using this method, I2C errors freely defined by HAL implementations"] # [doc = " can be converted to a set of generic I2C errors upon which generic"] # [doc = " code can act."] fn kind (& self) -> ErrorKind ; }
};
}

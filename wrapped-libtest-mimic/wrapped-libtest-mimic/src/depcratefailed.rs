// Generated macro for Failed (struct)
macro_rules! DepcrateFailed {
() => {
// Module: crate
// Provides: {"Failed"}
// Dependencies: {}
# [doc = " Indicates that a test/benchmark has failed. Optionally carries a message."] # [doc = ""] # [doc = " You usually want to use the `From` impl of this type, which allows you to"] # [doc = " convert any `T: fmt::Display` (e.g. `String`, `&str`, ...) into `Failed`."] # [derive (Debug , Clone)] pub struct Failed { msg : Option < String > , }
};
}

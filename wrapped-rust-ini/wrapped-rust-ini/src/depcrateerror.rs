// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error while parsing an INI document"] # [derive (Debug)] pub enum Error { Io (io :: Error) , Parse (ParseError) , }
};
}

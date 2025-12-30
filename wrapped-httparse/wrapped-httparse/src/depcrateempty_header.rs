// Generated macro for EMPTY_HEADER (const)
macro_rules! DepcrateEMPTY_HEADER {
() => {
// Module: crate
// Provides: {"EMPTY_HEADER"}
// Dependencies: {}
# [doc = " An empty header, useful for constructing a `Header` array to pass in for"] # [doc = " parsing."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let headers = [httparse::EMPTY_HEADER; 64];"] # [doc = " ```"] pub const EMPTY_HEADER : Header < 'static > = Header { name : "" , value : b"" } ;
};
}

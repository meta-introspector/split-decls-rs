// Generated macro for PortBuilderRepr (enum)
macro_rules! Depcrate_buildPortBuilderRepr {
() => {
// Module: crate::build
// Provides: {"PortBuilderRepr"}
// Dependencies: {}
# [doc = " Internal representation of a port builder."] # [derive (Debug , Clone , Copy)] # [non_exhaustive] enum PortBuilderRepr < 'a > { # [doc = " Empty port."] Empty , # [doc = " Port as an integer."] # [doc = ""] # [doc = " Note that RFC 3986 accepts any number of digits as a port, but"] # [doc = " practically (at least in TCP/IP) `u16` is enough."] Integer (u16) , # [doc = " Port as a string."] String (& 'a str) , }
};
}

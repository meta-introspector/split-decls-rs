// Generated macro for macro_7252 (macro)
macro_rules! Depcrate_methodsmacro_7252 {
() => {
// Module: crate::methods
// Provides: {"macro_7252"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for IP addresses that could be replaced with predefined constants such as"] # [doc = " `Ipv4Addr::new(127, 0, 0, 1)` instead of using the appropriate constants."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using specific IP addresses like `127.0.0.1` or `::1` is less clear and less maintainable than using the"] # [doc = " predefined constants `Ipv4Addr::LOCALHOST` or `Ipv6Addr::LOCALHOST`. These constants improve code"] # [doc = " readability, make the intent explicit, and are less error-prone."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::net::{Ipv4Addr, Ipv6Addr};"] # [doc = ""] # [doc = " // IPv4 loopback"] # [doc = " let addr_v4 = Ipv4Addr::new(127, 0, 0, 1);"] # [doc = ""] # [doc = " // IPv6 loopback"] # [doc = " let addr_v6 = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::net::{Ipv4Addr, Ipv6Addr};"] # [doc = ""] # [doc = " // IPv4 loopback"] # [doc = " let addr_v4 = Ipv4Addr::LOCALHOST;"] # [doc = ""] # [doc = " // IPv6 loopback"] # [doc = " let addr_v6 = Ipv6Addr::LOCALHOST;"] # [doc = " ```"] # [clippy :: version = "1.89.0"] pub IP_CONSTANT , pedantic , "hardcoded localhost IP address" }
};
}

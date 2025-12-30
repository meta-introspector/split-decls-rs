// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { enum IpAddr { V4 (u8 , u8 , u8 , u8) , V6 (String) , } let home = IpAddr :: V4 (127 , 0 , 0 , 1) ; let loopback = IpAddr :: V6 (String :: from ("::1")) ; }
};
}

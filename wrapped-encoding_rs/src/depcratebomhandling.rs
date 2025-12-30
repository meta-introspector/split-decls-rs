// Generated macro for BomHandling (enum)
macro_rules! DepcrateBomHandling {
() => {
// Module: crate
// Provides: {"BomHandling"}
// Dependencies: {}
# [doc = " Communicate the BOM handling mode."] # [derive (Debug , Copy , Clone)] enum BomHandling { # [doc = " Don't handle the BOM"] Off , # [doc = " Sniff for UTF-8, UTF-16BE or UTF-16LE BOM"] Sniff , # [doc = " Remove the BOM only if it's the BOM for this encoding"] Remove , }
};
}

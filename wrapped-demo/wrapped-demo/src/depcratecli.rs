// Generated macro for Cli (struct)
macro_rules! DepcrateCli {
() => {
// Module: crate
// Provides: {"Cli"}
// Dependencies: {}
# [doc = " Demo"] # [derive (Debug , Parser)] struct Cli { # [doc = " time in ms between two ticks."] # [arg (short , long , default_value_t = 250)] tick_rate : u64 , # [doc = " whether unicode symbols are used to improve the overall look of the app"] # [arg (short , long , default_value_t = true)] unicode : bool , }
};
}

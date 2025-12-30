// Generated macro for Ticker (struct)
macro_rules! DepcrateTicker {
() => {
// Module: crate
// Provides: {"Ticker"}
// Dependencies: {}
# [doc = " Runs task one by one."] struct Ticker < 'a > { # [doc = " The executor state."] state : & 'a State , # [doc = " Set to a non-zero sleeper ID when in sleeping state."] # [doc = ""] # [doc = " States a ticker can be in:"] # [doc = " 1) Woken."] # [doc = "    2a) Sleeping and unnotified."] # [doc = "    2b) Sleeping and notified."] sleeping : usize , }
};
}

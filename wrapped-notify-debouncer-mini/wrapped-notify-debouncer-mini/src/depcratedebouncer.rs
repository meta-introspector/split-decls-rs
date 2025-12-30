// Generated macro for Debouncer (struct)
macro_rules! DepcrateDebouncer {
() => {
// Module: crate
// Provides: {"Debouncer"}
// Dependencies: {}
# [doc = " Debouncer guard, stops the debouncer on drop"] # [derive (Debug)] pub struct Debouncer < T : Watcher > { watcher : T , stop_channel : Sender < InnerEvent > , }
};
}

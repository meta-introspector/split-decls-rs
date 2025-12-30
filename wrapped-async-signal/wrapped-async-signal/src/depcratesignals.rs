// Generated macro for Signals (struct)
macro_rules! DepcrateSignals {
() => {
// Module: crate
// Provides: {"Signals"}
// Dependencies: {}
# [doc = " Wait for a specific set of signals."] # [doc = ""] # [doc = " See the [module-level documentation](index.html) for more details."] pub struct Signals { # [doc = " The strategy used to read the signals."] notifier : sys :: Notifier , # [doc = " The map between signal numbers and signal IDs."] signal_ids : HashMap < Signal , SigId > , }
};
}

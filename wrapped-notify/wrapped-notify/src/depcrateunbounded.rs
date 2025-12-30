// Generated macro for unbounded (function)
macro_rules! Depcrateunbounded {
() => {
// Module: crate
// Provides: {"unbounded"}
// Dependencies: {}
# [inline] pub (crate) fn unbounded < T > () -> (Sender < T > , Receiver < T >) { std :: sync :: mpsc :: channel () }
};
}

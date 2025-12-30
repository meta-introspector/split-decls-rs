// Generated macro for macro_2680 (macro)
macro_rules! Depcrate_abortablemacro_2680 {
() => {
// Module: crate::abortable
// Provides: {"macro_2680"}
// Dependencies: {}
pin_project ! { # [doc = " A future/stream which can be remotely short-circuited using an `AbortHandle`."] # [derive (Debug , Clone)] # [must_use = "futures/streams do nothing unless you poll them"] pub struct Abortable < T > { # [pin] task : T , inner : Arc < AbortInner >, } }
};
}

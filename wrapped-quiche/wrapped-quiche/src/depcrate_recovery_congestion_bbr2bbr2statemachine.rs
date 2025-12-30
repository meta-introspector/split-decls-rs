// Generated macro for BBR2StateMachine (enum)
macro_rules! Depcrate_recovery_congestion_bbr2BBR2StateMachine {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"BBR2StateMachine"}
// Dependencies: {}
# [doc = " BBR2 Internal State Machine."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] enum BBR2StateMachine { Startup , Drain , ProbeBWDOWN , ProbeBWCRUISE , ProbeBWREFILL , ProbeBWUP , ProbeRTT , }
};
}

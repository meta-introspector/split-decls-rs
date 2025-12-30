// Generated macro for mtud_probes_include_immediate_ack (function)
macro_rules! Depcrate_testsmtud_probes_include_immediate_ack {
() => {
// Module: crate::tests
// Provides: {"mtud_probes_include_immediate_ack"}
// Dependencies: {}
# [test] fn mtud_probes_include_immediate_ack () { let _guard = subscribe () ; let mut pair = Pair :: default () ; let (client_ch , _) = pair . connect () ; pair . drive () ; let stats = pair . client_conn_mut (client_ch) . stats () ; assert_eq ! (stats . path . sent_plpmtud_probes , 4) ; assert_eq ! (stats . frame_tx . ping , 4) ; assert_eq ! (stats . frame_tx . immediate_ack , 4) ; }
};
}

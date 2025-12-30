// Generated macro for rustc_cfg_value (function)
macro_rules! Depcrate_outputrustc_cfg_value {
() => {
// Module: crate::output
// Provides: {"rustc_cfg_value"}
// Dependencies: {}
# [doc = " Like [`rustc_cfg`], but with the value specified separately."] # [doc = ""] # [doc = " To replace the"] # [doc = " less convenient `rustc_cfg(r#\"my_component=\"foo\"\"#)`, you can instead use"] # [doc = " `rustc_cfg_value(\"my_component\", \"foo\")`."] # [track_caller] pub fn rustc_cfg_value (key : & str , value : & str) { if ! is_ident (key) { panic ! ("cannot emit rustc-cfg-value: invalid key") ; } let value = value . escape_default () ; emit ("rustc-cfg" , format_args ! ("{key}=\"{value}\"")) ; }
};
}

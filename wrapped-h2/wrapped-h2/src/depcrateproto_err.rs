// Generated macro for proto_err (macro)
macro_rules! Depcrateproto_err {
() => {
// Module: crate
// Provides: {"proto_err"}
// Dependencies: {}
macro_rules ! proto_err { (conn : $ ($ msg : tt) +) => { tracing :: debug ! ("connection error PROTOCOL_ERROR -- {};" , format_args ! ($ ($ msg) +)) } ; (stream : $ ($ msg : tt) +) => { tracing :: debug ! ("stream error PROTOCOL_ERROR -- {};" , format_args ! ($ ($ msg) +)) } ; }
};
}

// Generated macro for message_expansion_error (function)
macro_rules! Depcrate_messagemessage_expansion_error {
() => {
// Module: crate::message
// Provides: {"message_expansion_error"}
// Dependencies: {}
# [doc = " Prints an error from `cargo expand` invocation."] # [doc = " Makes some suggestions when possible."] pub (crate) fn message_expansion_error (msg : Vec < u8 >) { let msg = String :: from_utf8 (msg) ; eprintln ! ("Expansion error:") ; if let Ok (msg) = msg { eprintln ! ("{}" , msg) ; if msg . contains ("no such subcommand: `expand`") { eprintln ! ("Perhaps, `cargo expand` is not installed?") ; eprintln ! ("Install it by running:") ; eprintln ! () ; eprintln ! ("\tcargo install cargo-expand") ; eprintln ! () ; } } else { eprintln ! ("<unprintable>") ; } }
};
}

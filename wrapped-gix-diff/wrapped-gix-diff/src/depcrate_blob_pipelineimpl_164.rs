// Generated macro for impl_164 (impl)
macro_rules! Depcrate_blob_pipelineimpl_164 {
() => {
// Module: crate::blob::pipeline
// Provides: {"impl_164"}
// Dependencies: {}
impl Driver { # [doc = " Produce an invocable command pre-configured to produce the filtered output on stdout after reading `path`."] pub fn prepare_binary_to_text_cmd (& self , path : & Path) -> Option < std :: process :: Command > { let command : & BStr = self . binary_to_text_command . as_ref () ? . as_ref () ; let cmd = gix_command :: prepare (gix_path :: from_bstr (command) . into_owned ()) . with_context (Default :: default ()) . with_shell () . stdin (Stdio :: null ()) . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . arg (path) . into () ; Some (cmd) } }
};
}

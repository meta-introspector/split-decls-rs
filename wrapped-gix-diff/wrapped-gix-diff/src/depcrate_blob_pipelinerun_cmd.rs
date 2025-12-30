// Generated macro for run_cmd (function)
macro_rules! Depcrate_blob_pipelinerun_cmd {
() => {
// Module: crate::blob::pipeline
// Provides: {"run_cmd"}
// Dependencies: {}
fn run_cmd (rela_path : & BStr , mut cmd : Command , out : & mut Vec < u8 >) -> Result < () , convert_to_diffable :: Error > { gix_trace :: debug ! (cmd = ? cmd , "Running binary-to-text command") ; let mut res = cmd . output () . map_err (| err | convert_to_diffable :: Error :: RunTextConvFilter { rela_path : rela_path . to_owned () , cmd : format ! ("{cmd:?}") , source : err , }) ? ; if ! res . status . success () { return Err (convert_to_diffable :: Error :: TextConvFilterFailed { rela_path : rela_path . to_owned () , cmd : format ! ("{cmd:?}") , stderr : res . stderr . into () , }) ; } out . append (& mut res . stdout) ; Ok (()) }
};
}

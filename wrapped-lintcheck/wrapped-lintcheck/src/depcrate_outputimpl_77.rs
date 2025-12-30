// Generated macro for impl_77 (impl)
macro_rules! Depcrate_outputimpl_77 {
() => {
// Module: crate::output
// Provides: {"impl_77"}
// Dependencies: {}
impl RustcIce { pub fn from_stderr_and_status (crate_name : & str , status : ExitStatus , stderr : & str) -> Option < Self > { if status . code () . unwrap_or (0) == 101 { Some (Self { crate_name : crate_name . to_owned () , ice_content : stderr . to_owned () , }) } else { None } } }
};
}

// Generated macro for raw (function)
macro_rules! Depcrate_helper_invokeraw {
() => {
// Module: crate::helper::invoke
// Provides: {"raw"}
// Dependencies: {}
pub (crate) fn raw (helper : & mut crate :: Program , action : & Action) -> std :: result :: Result < Option < Vec < u8 > > , Error > { let (mut stdin , stdout) = helper . start (action) ? ; if let (Action :: Get (_) , None) = (& action , & stdout) { panic ! ("BUG: `Helper` impls must return an output handle to read output from if Action::Get is provided") } action . send (& mut stdin) ? ; drop (stdin) ; let stdout = stdout . map (| mut stdout | { let mut buf = Vec :: new () ; stdout . read_to_end (& mut buf) . map (| _ | buf) }) . transpose () . map_err (| err | Error :: CredentialsHelperFailed { source : err }) ? ; helper . finish () . map_err (| err | { if err . kind () == std :: io :: ErrorKind :: Other { Error :: CredentialsHelperFailed { source : err } } else { err . into () } }) ? ; match matches ! (action , Action :: Get (_)) . then (| | stdout) . flatten () { None => Ok (None) , Some (stdout) => Ok (Some (stdout)) , } }
};
}

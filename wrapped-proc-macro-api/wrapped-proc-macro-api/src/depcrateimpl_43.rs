// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl ProcMacroClient { # [doc = " Spawns an external process as the proc macro server and returns a client connected to it."] pub fn spawn < 'a > (process_path : & AbsPath , env : impl IntoIterator < Item = (impl AsRef < std :: ffi :: OsStr > , & 'a Option < impl 'a + AsRef < std :: ffi :: OsStr > >) , > + Clone ,) -> io :: Result < ProcMacroClient > { let process = ProcMacroServerProcess :: run (process_path , env) ? ; Ok (ProcMacroClient { process : Arc :: new (process) , path : process_path . to_owned () }) } # [doc = " Returns the absolute path to the proc-macro server."] pub fn server_path (& self) -> & AbsPath { & self . path } # [doc = " Loads a proc-macro dylib into the server process returning a list of `ProcMacro`s loaded."] pub fn load_dylib (& self , dylib : MacroDylib) -> Result < Vec < ProcMacro > , ServerError > { let _p = tracing :: info_span ! ("ProcMacroServer::load_dylib") . entered () ; let macros = self . process . find_proc_macros (& dylib . path) ? ; let dylib_path = Arc :: new (dylib . path) ; let dylib_last_modified = std :: fs :: metadata (dylib_path . as_path ()) . ok () . and_then (| metadata | metadata . modified () . ok ()) ; match macros { Ok (macros) => Ok (macros . into_iter () . map (| (name , kind) | ProcMacro { process : self . process . clone () , name : name . into () , kind , dylib_path : dylib_path . clone () , dylib_last_modified , }) . collect ()) , Err (message) => Err (ServerError { message , io : None }) , } } # [doc = " Checks if the proc-macro server has exited."] pub fn exited (& self) -> Option < & ServerError > { self . process . exited () } }
};
}

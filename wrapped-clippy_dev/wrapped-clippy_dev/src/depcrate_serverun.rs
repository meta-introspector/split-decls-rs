// Generated macro for run (function)
macro_rules! Depcrate_serverun {
() => {
// Module: crate::serve
// Provides: {"run"}
// Dependencies: {}
# [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the python commands could not be spawned"] pub fn run (port : u16 , lint : Option < String >) -> ! { let mut url = Some (match lint { None => format ! ("http://localhost:{port}") , Some (lint) => format ! ("http://localhost:{port}/#{lint}") , }) ; let mut last_update = mtime ("util/gh-pages/index.html") ; loop { if is_metadata_outdated (mem :: replace (& mut last_update , SystemTime :: now ())) { let _ = expect_action (cargo_cmd () . arg ("collect-metadata") . status () , ErrAction :: Run , "cargo collect-metadata" ,) ; last_update = SystemTime :: now () ; } if let Some (url) = url . take () { thread :: spawn (move | | { let mut child = expect_action (Command :: new (PYTHON) . args (["-m" , "http.server" , port . to_string () . as_str ()]) . current_dir ("util/gh-pages") . spawn () , ErrAction :: Run , "python -m http.server" ,) ; thread :: sleep (Duration :: from_millis (500)) ; let _result = opener :: open (url) ; expect_action (child . wait () , ErrAction :: Run , "python -m http.server") ; }) ; } thread :: sleep (Duration :: from_millis (1000)) ; } }
};
}

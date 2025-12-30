// Generated macro for build_library (function)
macro_rules! Depcratebuild_library {
() => {
// Module: crate
// Provides: {"build_library"}
// Dependencies: {}
fn build_library (output : & std :: path :: Path , library : & str , functions : & BTreeMap < String , CallingConvention > ,) { let mut path = std :: path :: PathBuf :: from (output) ; path . push (format ! ("{library}.c")) ; let mut c = std :: fs :: File :: create (& path) . unwrap () ; path . pop () ; path . push (format ! ("{library}.def")) ; let mut def = std :: fs :: File :: create (& path) . unwrap () ; def . write_all (format ! (r#"
LIBRARY {library}
EXPORTS
"#) . as_bytes () ,) . unwrap () ; for (function , calling_convention) in functions { let buffer = match calling_convention { CallingConvention :: Stdcall (size) => { let mut buffer = format ! ("void __stdcall {function}(") ; for param in 0 .. (* size / 4) { use std :: fmt :: Write ; write ! (& mut buffer , "int p{param}, ") . unwrap () ; } if buffer . ends_with (' ') { buffer . truncate (buffer . len () - 2) ; } buffer . push_str (") {}\n") ; buffer } CallingConvention :: Cdecl => { format ! ("void __cdecl {function}() {{}}\n") } } ; c . write_all (buffer . as_bytes ()) . unwrap () ; def . write_all (format ! ("{function}\n") . as_bytes ()) . unwrap () ; } drop (c) ; drop (def) ; let mut cmd = std :: process :: Command :: new ("cl") ; cmd . current_dir (output) ; cmd . arg ("/nologo") ; cmd . arg ("/c") ; cmd . arg (format ! ("{library}.c")) ; cmd . output () . unwrap () ; let mut cmd = std :: process :: Command :: new ("lib") ; cmd . current_dir (output) ; cmd . arg ("/nologo") ; cmd . arg (format ! ("/out:{library}.lib")) ; cmd . arg (format ! ("/def:{library}.def")) ; cmd . arg (format ! ("{library}.obj")) ; cmd . output () . unwrap () ; path . pop () ; path . push (format ! ("{library}.c")) ; path . pop () ; path . push (format ! ("{library}.def")) ; std :: fs :: remove_file (& path) . unwrap () ; path . pop () ; path . push (format ! ("{library}.exp")) ; std :: fs :: remove_file (& path) . unwrap_or_else (| _ | panic ! ("{path:?}")) ; path . pop () ; path . push (format ! ("{library}.obj")) ; std :: fs :: remove_file (& path) . unwrap () ; }
};
}

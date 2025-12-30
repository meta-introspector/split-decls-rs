// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let config = Config :: parse_args () ; println ! ("starting test server") ; let listener = bind_socket (config . bind) ; let (work , tmp) : (PathBuf , PathBuf) = if cfg ! (target_os = "android") { ("/data/local/tmp/work" . into () , "/data/local/tmp/work/tmp" . into ()) } else { let mut work_dir = env :: temp_dir () ; work_dir . push ("work") ; let mut tmp_dir = work_dir . clone () ; tmp_dir . push ("tmp") ; (work_dir , tmp_dir) } ; println ! ("listening on {}!" , config . bind) ; t ! (fs :: create_dir_all (& work)) ; t ! (fs :: create_dir_all (& tmp)) ; let lock = Arc :: new (Mutex :: new (())) ; for socket in listener . incoming () { let mut socket = t ! (socket) ; let mut buf = [0 ; 4] ; if socket . read_exact (& mut buf) . is_err () { continue ; } if & buf [..] == b"ping" { print_verbose ("Received ping" , config) ; t ! (socket . write_all (b"pong")) ; } else if & buf [..] == b"push" { handle_push (socket , & work , config) ; } else if & buf [..] == b"run " { let lock = lock . clone () ; let work = work . clone () ; let tmp = tmp . clone () ; let f = move | | handle_run (socket , & work , & tmp , & lock , config) ; if config . sequential { f () ; } else { thread :: spawn (f) ; } } else { panic ! ("unknown command {:?}" , buf) ; } } }
};
}

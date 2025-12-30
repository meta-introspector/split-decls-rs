// Generated macro for show_help (function)
macro_rules! Depcrateshow_help {
() => {
// Module: crate
// Provides: {"show_help"}
// Dependencies: {}
fn show_help () { eprintln ! (r#"Usage:

{} [OPTIONS]

OPTIONS:
    --bind <IP>:<PORT>   Specify IP address and port to listen for requests, e.g. "0.0.0.0:12345"
    --sequential         Run only one test at a time
    --batch              Send stdout and stderr in batch instead of streaming
    -v, --verbose        Show status messages
    -h, --help           Show this help screen
"# , std :: env :: args () . next () . unwrap ()) ; }
};
}

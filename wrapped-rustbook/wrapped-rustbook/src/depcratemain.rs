// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let crate_version = concat ! ("v" , crate_version ! ()) ; env_logger :: Builder :: from_env (env_logger :: Env :: default () . default_filter_or ("warn")) . init () ; let d_arg = arg ! (- d -- "dest-dir" < DEST_DIR > "The output directory for your book\n(Defaults to ./book when omitted)") . required (false) . value_parser (clap :: value_parser ! (PathBuf)) ; let l_arg = arg ! (- l -- "lang" < LANGUAGE > "The output language") . required (false) . value_parser (clap :: value_parser ! (String)) ; let root_arg = arg ! (-- "rust-root" < ROOT_DIR > "Path to the root of the rust source tree") . required (false) . value_parser (clap :: value_parser ! (PathBuf)) ; let dir_arg = arg ! ([dir] "Root directory for the book\n\
                              (Defaults to the current directory when omitted)") . value_parser (clap :: value_parser ! (PathBuf)) ; let library_path_arg = arg ! (- L -- "library-path" < PATHS > "A comma-separated list of directories to add to the crate search\n\
        path when building tests") . required (false) . value_parser (parse_library_paths) ; let matches = Command :: new ("rustbook") . about ("Build a book with mdBook") . author ("Steve Klabnik <steve@steveklabnik.com>") . version (crate_version) . subcommand_required (true) . arg_required_else_help (true) . subcommand (Command :: new ("build") . about ("Build the book from the markdown files") . arg (d_arg) . arg (l_arg) . arg (root_arg) . arg (& dir_arg) ,) . subcommand (Command :: new ("test") . about ("Tests that a book's Rust code samples compile") . arg (dir_arg) . arg (library_path_arg) ,) . get_matches () ; match matches . subcommand () { Some (("build" , sub_matches)) => { if let Err (e) = build (sub_matches) { handle_error (e) ; } } Some (("test" , sub_matches)) => { if let Err (e) = test (sub_matches) { handle_error (e) ; } } _ => unreachable ! () , } ; }
};
}

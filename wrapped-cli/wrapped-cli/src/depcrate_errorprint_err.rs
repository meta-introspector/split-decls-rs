// Generated macro for print_err (function)
macro_rules! Depcrate_errorprint_err {
() => {
// Module: crate::error
// Provides: {"print_err"}
// Dependencies: {}
# [doc = " Pretty-prints `message` to `stderr`."] # [doc = " If `context` is supplied,"] # [doc = " it will be appended to the first line."] pub fn print_err (message : & str , context : Option < String >) { if let Some (context) = context { eprintln ! ("{} {}:" , "An error occurred" . red () , context . red ()) ; } else { eprintln ! ("{}" , "An error occurred:" . red ()) ; } eprintln ! ("\t{}" , message . red () . bold ()) ; }
};
}

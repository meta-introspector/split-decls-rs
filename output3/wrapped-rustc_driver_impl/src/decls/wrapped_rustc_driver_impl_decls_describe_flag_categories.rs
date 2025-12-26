use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Show help for flag categories shared between rustdoc and rustc.
///
/// Returns whether a help option was printed.
pub fn describe_flag_categories(early_dcx: &EarlyDiagCtxt, matches: &Matches) -> bool {
    let wall = matches.opt_strs("W");
    if wall.iter().any(|x| *x == "all") {
        print_wall_help();
        return true;
    }
    let debug_flags = matches.opt_strs("Z");
    if debug_flags.iter().any(|x| *x == "help") {
        describe_debug_flags();
        return true;
    }
    let cg_flags = matches.opt_strs("C");
    if cg_flags.iter().any(|x| *x == "help") {
        describe_codegen_flags();
        return true;
    }
    if cg_flags.iter().any(|x| *x == "passes=list") {
        get_backend_from_raw_matches(early_dcx, matches).print_passes();
        return true;
    }
    false
}

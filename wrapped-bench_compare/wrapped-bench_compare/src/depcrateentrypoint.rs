// Generated macro for entrypoint (function)
macro_rules! Depcrateentrypoint {
() => {
// Module: crate
// Provides: {"entrypoint"}
// Dependencies: {}
fn entrypoint () -> Result < () , Error > { let config : Config = toml :: from_str (& std :: fs :: read_to_string ("bench_compare.toml") . unwrap ()) . unwrap () ; if config . parsers . is_empty () { println ! ("Please add at least one parser. Refer to the README for instructions.") ; return Ok (()) ; } let args : Vec < _ > = std :: env :: args () . collect () ; if args . len () != 2 || (args . len () == 2 && ! ["time_parse" , "run_bench"] . contains (& args [1] . as_str ())) { println ! ("Usage: bench_compare <time_parse|run_bench>") ; return Ok (()) ; } match args [1] . as_str () { "run_bench" => run_bench (& config) ? , "time_parse" => unimplemented ! () , _ => unreachable ! () , } Ok (()) }
};
}

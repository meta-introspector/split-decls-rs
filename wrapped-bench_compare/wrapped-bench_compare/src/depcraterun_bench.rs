// Generated macro for run_bench (function)
macro_rules! Depcraterun_bench {
() => {
// Module: crate
// Provides: {"run_bench"}
// Dependencies: {}
# [doc = " Run the `run_bench` binary on the given parsers."] fn run_bench (config : & Config) -> Result < () , Error > { std :: fs :: create_dir_all (& config . yaml_output_dir) ? ; let inputs = list_input_files (config) ? ; let iterations = format ! ("{}" , config . iterations) ; let mut averages = vec ! [] ; for input in & inputs { let input_basename = Path :: new (& input) . file_name () . unwrap () . to_string_lossy () ; let mut input_times = vec ! [] ; for parser in & config . parsers { println ! ("Running {input_basename} against {}" , parser . name) ; let path = Path :: new (& parser . path) . join ("run_bench") ; let output = std :: process :: Command :: new (path) . arg (input) . arg (& iterations) . arg ("--output-yaml") . output () ? ; if output . status . code () . unwrap_or (1) == 0 { let s = String :: from_utf8_lossy (& output . stdout) ; match serde_yaml :: from_str :: < BenchYamlOutput > (& s) { Ok (output) => { input_times . push (output . average) ; serde_yaml :: to_writer (BufWriter :: new (File :: create (format ! ("{}/{}-{}" , config . yaml_output_dir , parser . name , input_basename)) ?) , & output ,) ? ; } Err (e) => { println ! ("Errored: Invalid YAML output: {e}") ; input_times . push (0) ; } } } else { println ! ("Errored: process did exit non-zero") ; input_times . push (0) ; } } averages . push (input_times) ; } save_run_bench_csv (config , & inputs , & averages) }
};
}

// Generated macro for save_run_bench_csv (function)
macro_rules! Depcratesave_run_bench_csv {
() => {
// Module: crate
// Provides: {"save_run_bench_csv"}
// Dependencies: {}
# [doc = " Save a CSV file with all averages from `run_bench`."] fn save_run_bench_csv (config : & Config , inputs : & [String] , averages : & [Vec < u64 >] ,) -> Result < () , Error > { let mut csv = BufWriter :: new (File :: create (& config . csv_output) ?) ; for parser in & config . parsers { write ! (csv , ",{}" , parser . name ,) ? ; } writeln ! (csv) ? ; for (path , averages) in inputs . iter () . zip (averages . iter ()) { let filename = Path :: new (path) . file_name () . unwrap () . to_string_lossy () ; write ! (csv , "{}" , filename) ? ; for avg in averages { write ! (csv , ",{avg}") ? ; } writeln ! (csv) ? ; } Ok (()) }
};
}

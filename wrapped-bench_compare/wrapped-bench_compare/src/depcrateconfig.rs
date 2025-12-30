// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " General configuration structure."] # [derive (Serialize , Deserialize)] struct Config { # [doc = " The path to the directory containing the input yaml files."] yaml_input_dir : String , # [doc = " Number of iterations to run, if using `run_bench`."] iterations : u32 , # [doc = " The parsers to run."] parsers : Vec < Parser > , # [doc = " The path to the directory in which `run_bench`'s yamls are saved."] yaml_output_dir : String , # [doc = " The path to the CSV output aggregating times for each parser and file."] csv_output : String , }
};
}

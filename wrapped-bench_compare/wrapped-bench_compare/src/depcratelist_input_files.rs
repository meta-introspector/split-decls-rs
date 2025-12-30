// Generated macro for list_input_files (function)
macro_rules! Depcratelist_input_files {
() => {
// Module: crate
// Provides: {"list_input_files"}
// Dependencies: {}
# [doc = " Returns the paths to the input yaml files."] fn list_input_files (config : & Config) -> Result < Vec < String > , Error > { Ok (std :: fs :: read_dir (& config . yaml_input_dir) ? . filter_map (Result :: ok) . map (| entry | entry . path () . to_string_lossy () . to_string ()) . filter (| path | { Path :: new (path) . extension () . map_or (false , | ext | ext . eq_ignore_ascii_case ("yaml")) }) . collect ()) }
};
}

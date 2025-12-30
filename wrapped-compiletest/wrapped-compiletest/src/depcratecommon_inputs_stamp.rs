// Generated macro for common_inputs_stamp (function)
macro_rules! Depcratecommon_inputs_stamp {
() => {
// Module: crate
// Provides: {"common_inputs_stamp"}
// Dependencies: {}
# [doc = " Returns the most recent last-modified timestamp from among the input files"] # [doc = " that are considered relevant to all tests (e.g. the compiler, std, and"] # [doc = " compiletest itself)."] # [doc = ""] # [doc = " (Some of these inputs aren't actually relevant to _all_ tests, but they are"] # [doc = " common to some subset of tests, and are hopefully unlikely to be modified"] # [doc = " while working on other tests.)"] fn common_inputs_stamp (config : & Config) -> Stamp { let src_root = & config . src_root ; let mut stamp = Stamp :: from_path (& config . rustc_path) ; let pretty_printer_files = ["src/etc/rust_types.py" , "src/etc/gdb_load_rust_pretty_printers.py" , "src/etc/gdb_lookup.py" , "src/etc/gdb_providers.py" , "src/etc/lldb_batchmode.py" , "src/etc/lldb_lookup.py" , "src/etc/lldb_providers.py" ,] ; for file in & pretty_printer_files { let path = src_root . join (file) ; stamp . add_path (& path) ; } stamp . add_dir (& src_root . join ("src/etc/natvis")) ; stamp . add_dir (& config . run_lib_path) ; if let Some (ref rustdoc_path) = config . rustdoc_path { stamp . add_path (& rustdoc_path) ; stamp . add_path (& src_root . join ("src/etc/htmldocck.py")) ; } if let Some (coverage_dump_path) = & config . coverage_dump_path { stamp . add_path (coverage_dump_path) } stamp . add_dir (& src_root . join ("src/tools/run-make-support")) ; stamp . add_dir (& src_root . join ("src/tools/compiletest")) ; stamp }
};
}

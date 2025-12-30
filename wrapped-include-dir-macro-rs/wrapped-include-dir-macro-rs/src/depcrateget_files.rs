// Generated macro for get_files (function)
macro_rules! Depcrateget_files {
() => {
// Module: crate
// Provides: {"get_files"}
// Dependencies: {}
fn get_files < P : AsRef < Path > > (dir : P) -> Vec < PathBuf > { let mut files = vec ! [] ; let listing : Vec < _ > = :: std :: fs :: read_dir (dir) . expect ("could not read directory") . map (| entry | entry . unwrap () . path ()) . collect () ; for path in listing { if path . is_file () { files . push (path) } else if path . is_dir () { for file in get_files (& path) { files . push (file . into ()) } } } files }
};
}

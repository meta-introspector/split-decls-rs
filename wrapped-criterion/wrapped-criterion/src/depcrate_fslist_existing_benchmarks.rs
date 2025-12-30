// Generated macro for list_existing_benchmarks (function)
macro_rules! Depcrate_fslist_existing_benchmarks {
() => {
// Module: crate::fs
// Provides: {"list_existing_benchmarks"}
// Dependencies: {}
pub fn list_existing_benchmarks < P > (directory : & P) -> Result < Vec < BenchmarkId > > where P : AsRef < Path > , { fn is_benchmark (entry : & DirEntry) -> bool { entry . file_name () == OsStr :: new ("benchmark.json") && entry . path () . parent () . unwrap () . file_name () . unwrap () == OsStr :: new ("new") } let mut ids = vec ! [] ; for entry in WalkDir :: new (directory) . into_iter () . filter_map (:: std :: result :: Result :: ok) . filter (is_benchmark) { let id : BenchmarkId = load (entry . path ()) ? ; ids . push (id) ; } Ok (ids) }
};
}

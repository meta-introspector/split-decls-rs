// Generated macro for run (function)
macro_rules! Depcrate_organizerun {
() => {
// Module: crate::organize
// Provides: {"run"}
// Dependencies: {}
pub fn run < P : NestedProgress > (mode : Mode , source_dir : impl AsRef < Path > , destination : impl AsRef < Path > , mut progress : P , threads : Option < usize > ,) -> anyhow :: Result < () > { let mut num_errors = 0usize ; let destination = destination . as_ref () . canonicalize () ? ; for (path_to_move , kind) in find_git_repository_workdirs (source_dir , progress . add_child ("Searching repositories") , false , threads) { if let Err (err) = handle (mode , kind , & path_to_move , & destination , & mut progress) { progress . fail (format ! ("Error when handling directory {:?}: {}" , path_to_move . display () , err)) ; num_errors += 1 ; } } if num_errors > 0 { anyhow :: bail ! ("Failed to handle {num_errors} repositories") } else { Ok (()) } }
};
}

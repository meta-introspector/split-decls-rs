// Generated macro for symlink_dir_force (function)
macro_rules! Depcrate_core_build_steps_docsymlink_dir_force {
() => {
// Module: crate::core::build_steps::doc
// Provides: {"symlink_dir_force"}
// Dependencies: {}
fn symlink_dir_force (config : & Config , original : & Path , link : & Path) { if config . dry_run () { return ; } if let Ok (m) = fs :: symlink_metadata (link) { if m . file_type () . is_dir () { t ! (fs :: remove_dir_all (link)) ; } else { t ! (fs :: remove_file (link) . or_else (| _ | fs :: remove_dir (link))) ; } } t ! (symlink_dir (config , original , link) , format ! ("failed to create link from {} -> {}" , link . display () , original . display ())) ; }
};
}

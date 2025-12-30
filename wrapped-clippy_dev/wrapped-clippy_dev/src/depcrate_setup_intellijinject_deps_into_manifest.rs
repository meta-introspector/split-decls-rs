// Generated macro for inject_deps_into_manifest (function)
macro_rules! Depcrate_setup_intellijinject_deps_into_manifest {
() => {
// Module: crate::setup::intellij
// Provides: {"inject_deps_into_manifest"}
// Dependencies: {}
fn inject_deps_into_manifest (rustc_source_dir : & Path , manifest_path : & str , cargo_toml : & str , lib_rs : & str ,) -> std :: io :: Result < () > { if cargo_toml . contains (RUSTC_PATH_SECTION) { eprintln ! ("warn: dependencies are already setup inside {manifest_path}, skipping file") ; return Ok (()) ; } let extern_crates = lib_rs . lines () . filter (| line | line . starts_with ("extern crate rustc_")) . map (| s | & s [13 .. (s . len () - 1)]) ; let new_deps = extern_crates . map (| dep | { format ! ("{dep} = {{ path = \"{}/{dep}\" }}\n" , rustc_source_dir . display ()) }) ; let mut all_deps = String :: from ("[target.'cfg(NOT_A_PLATFORM)'.dependencies]\n") ; new_deps . for_each (| dep_line | { all_deps . push_str (& dep_line) ; }) ; all_deps . push_str ("\n[dependencies]\n") ; let new_manifest = cargo_toml . replacen ("[dependencies]\n" , & all_deps , 1) ; let mut file = File :: create (manifest_path) ? ; file . write_all (new_manifest . as_bytes ()) ? ; println ! ("info: successfully setup dependencies inside {manifest_path}") ; Ok (()) }
};
}

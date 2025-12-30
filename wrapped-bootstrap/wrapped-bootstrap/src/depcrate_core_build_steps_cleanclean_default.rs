// Generated macro for clean_default (function)
macro_rules! Depcrate_core_build_steps_cleanclean_default {
() => {
// Module: crate::core::build_steps::clean
// Provides: {"clean_default"}
// Dependencies: {}
fn clean_default (build : & Build) { rm_rf (& build . out . join ("tmp")) ; rm_rf (& build . out . join ("dist")) ; rm_rf (& build . out . join ("bootstrap") . join (".last-warned-change-id")) ; rm_rf (& build . out . join ("bootstrap-shims-dump")) ; rm_rf (BuildStamp :: new (& build . out) . with_prefix ("rustfmt") . path ()) ; let mut hosts : Vec < _ > = build . hosts . iter () . map (| t | build . out . join (t)) . collect () ; hosts . push (build . out . join ("host")) ; for host in hosts { let entries = match host . read_dir () { Ok (iter) => iter , Err (_) => continue , } ; for entry in entries { let entry = t ! (entry) ; if entry . file_name () . to_str () == Some ("llvm") { continue ; } let path = t ! (entry . path () . canonicalize ()) ; rm_rf (& path) ; } } }
};
}

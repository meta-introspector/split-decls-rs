// Generated macro for capture_vcs_info (function)
macro_rules! Depcratecapture_vcs_info {
() => {
// Module: crate
// Provides: {"capture_vcs_info"}
// Dependencies: {}
fn capture_vcs_info (ws_root : & Path , force : bool) -> Option < String > { let maybe_git = | command : & str | { Command :: new ("git") . current_dir (ws_root) . args (command . split_whitespace () . collect :: < Vec < _ > > ()) . output () . expect ("git should be installed") } ; assert ! (ws_root . join ("Cargo.toml") . exists ()) ; let relative = maybe_git ("ls-files --full-name Cargo.toml") ; if ! relative . status . success () { if ! force { panic ! ("git repository not detected, use -f to force") ; } return None ; } let p = Path :: new (std :: str :: from_utf8 (& relative . stdout) . unwrap () . trim ()) ; let relative = p . parent () . unwrap () ; if ! force { let has_changes = ! maybe_git ("diff-index --quiet HEAD .") . status . success () ; if has_changes { panic ! ("git repo appears to have changes, use -f to force, or clean the repo") ; } } let commit = maybe_git ("rev-parse HEAD") ; assert ! (commit . status . success ()) ; let commit = std :: str :: from_utf8 (& commit . stdout) . unwrap () . trim () ; let remote = maybe_git ("remote get-url origin") ; assert ! (remote . status . success ()) ; let remote = std :: str :: from_utf8 (& remote . stdout) . unwrap () . trim () ; let info = format ! ("{{\n  \"git\": {{\n    \"sha1\": \"{}\",\n     \"remote\": \"{}\"\n  }},\
         \n  \"path_in_vcs\": \"{}\"\n}}\n" , commit , remote , relative . display ()) ; eprintln ! ("recording vcs info:\n{}" , info) ; Some (info) }
};
}

// Generated macro for rustc_version_info_inner (function)
macro_rules! Depcrate_utilsrustc_version_info_inner {
() => {
// Module: crate::utils
// Provides: {"rustc_version_info_inner"}
// Dependencies: {}
fn rustc_version_info_inner (rustc : Option < & str > , toolchain : Option < & str > ,) -> Result < RustcVersionInfo , String > { let output = if let Some (toolchain) = toolchain { run_command (& [& rustc . unwrap_or ("rustc") , & toolchain , & "-vV"] , None) } else { run_command (& [& rustc . unwrap_or ("rustc") , & "-vV"] , None) } ? ; let content = std :: str :: from_utf8 (& output . stdout) . unwrap_or ("") ; let mut info = RustcVersionInfo :: default () ; let mut lines = content . split ('\n') ; info . short = match lines . next () { Some (s) => s . to_string () , None => return Err ("failed to retrieve rustc version" . to_string ()) , } ; for line in lines . map (| line | line . trim ()) { match line . split_once (':') { Some (("host" , data)) => info . host = Some (data . trim () . to_string ()) , Some (("release" , data)) => info . version = data . trim () . to_string () , Some (("commit-hash" , data)) => info . commit_hash = Some (data . trim () . to_string ()) , Some (("commit-date" , data)) => info . commit_date = Some (data . trim () . to_string ()) , _ => { } } } if info . version . is_empty () { Err ("failed to retrieve rustc version" . to_string ()) } else { Ok (info) } }
};
}

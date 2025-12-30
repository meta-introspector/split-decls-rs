// Generated macro for install (function)
macro_rules! Depcrate_npminstall {
() => {
// Module: crate::npm
// Provides: {"install"}
// Dependencies: {}
# [doc = " Install all the npm deps, and return the path of `node_modules`."] pub fn install (src_root_path : & Path , out_dir : & Path , npm : & Path) -> Result < PathBuf , io :: Error > { let nm_path = out_dir . join ("node_modules") ; let copy_to_build = | p | { fs :: copy (src_root_path . join (p) , out_dir . join (p)) . map_err (| e | { eprintln ! ("unable to copy {p:?} to build directory: {e:?}") ; e }) } ; copy_to_build ("package.json") ? ; copy_to_build ("package-lock.json") ? ; let mut cmd = Command :: new (npm) ; if CiEnv :: is_ci () { cmd . arg ("ci") ; } else { cmd . arg ("install") ; } cmd . args (& ["--audit=false" , "--save=false" , "--fund=false"]) ; cmd . current_dir (out_dir) ; let exit_status = cmd . spawn () ? . wait () ? ; if ! exit_status . success () { eprintln ! ("npm install did not exit successfully") ; return Err (io :: Error :: other (Box :: < dyn Error + Send + Sync > :: from (format ! ("npm install returned exit code {exit_status}")))) ; } Ok (nm_path) }
};
}

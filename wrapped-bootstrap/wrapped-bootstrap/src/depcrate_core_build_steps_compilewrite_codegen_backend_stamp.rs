// Generated macro for write_codegen_backend_stamp (function)
macro_rules! Depcrate_core_build_steps_compilewrite_codegen_backend_stamp {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"write_codegen_backend_stamp"}
// Dependencies: {}
# [doc = " Write filtered `files` into the passed build stamp and returns it."] fn write_codegen_backend_stamp (mut stamp : BuildStamp , files : Vec < PathBuf > , dry_run : bool ,) -> BuildStamp { if dry_run { return stamp ; } let mut files = files . into_iter () . filter (| f | { let filename = f . file_name () . unwrap () . to_str () . unwrap () ; is_dylib (f) && filename . contains ("rustc_codegen_") }) ; let codegen_backend = match files . next () { Some (f) => f , None => panic ! ("no dylibs built for codegen backend?") , } ; if let Some (f) = files . next () { panic ! ("codegen backend built two dylibs:\n{}\n{}" , codegen_backend . display () , f . display ()) ; } let codegen_backend = codegen_backend . to_str () . unwrap () ; stamp = stamp . add_stamp (codegen_backend) ; t ! (stamp . write ()) ; stamp }
};
}

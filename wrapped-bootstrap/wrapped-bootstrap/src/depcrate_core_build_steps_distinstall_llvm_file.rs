// Generated macro for install_llvm_file (function)
macro_rules! Depcrate_core_build_steps_distinstall_llvm_file {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"install_llvm_file"}
// Dependencies: {}
fn install_llvm_file (builder : & Builder < '_ > , source : & Path , destination : & Path , install_symlink : bool ,) { if builder . config . dry_run () { return ; } if source . is_symlink () { builder . install (& t ! (fs :: canonicalize (source)) , destination , FileType :: NativeLibrary) ; let full_dest = destination . join (source . file_name () . unwrap ()) ; if install_symlink { builder . copy_link (source , & full_dest , FileType :: NativeLibrary) ; } else { let link = t ! (fs :: read_link (source)) ; let mut linker_script = t ! (fs :: File :: create (full_dest)) ; t ! (write ! (linker_script , "INPUT({})\n" , link . display ())) ; let meta = t ! (fs :: metadata (source)) ; if let Ok (mtime) = meta . modified () { t ! (linker_script . set_modified (mtime)) ; } } } else { builder . install (source , destination , FileType :: NativeLibrary) ; } }
};
}

// Generated macro for fix_bin_or_dylib (function)
macro_rules! Depcrate_core_downloadfix_bin_or_dylib {
() => {
// Module: crate::core::download
// Provides: {"fix_bin_or_dylib"}
// Dependencies: {}
fn fix_bin_or_dylib (out : & Path , fname : & Path , exec_ctx : & ExecutionContext) { assert_eq ! (SHOULD_FIX_BINS_AND_DYLIBS . get () , Some (& true)) ; println ! ("attempting to patch {}" , fname . display ()) ; static NIX_DEPS_DIR : OnceLock < PathBuf > = OnceLock :: new () ; let mut nix_build_succeeded = true ; let nix_deps_dir = NIX_DEPS_DIR . get_or_init (| | { let nix_deps_dir = out . join (".nix-deps") ; const NIX_EXPR : & str = "
        with (import <nixpkgs> {});
        symlinkJoin {
            name = \"rust-stage0-dependencies\";
            paths = [
                zlib
                patchelf
                stdenv.cc.bintools
            ];
        }
        " ; nix_build_succeeded = command ("nix-build") . allow_failure () . args ([Path :: new ("-E") , Path :: new (NIX_EXPR) , Path :: new ("-o") , & nix_deps_dir]) . run_capture_stdout (exec_ctx) . is_success () ; nix_deps_dir }) ; if ! nix_build_succeeded { return ; } let mut patchelf = command (nix_deps_dir . join ("bin/patchelf")) ; patchelf . args (& [OsString :: from ("--add-rpath") , OsString :: from (t ! (fs :: canonicalize (nix_deps_dir)) . join ("lib")) ,]) ; if ! path_is_dylib (fname) { let dynamic_linker_path = nix_deps_dir . join ("nix-support/dynamic-linker") ; let dynamic_linker = t ! (fs :: read_to_string (dynamic_linker_path)) ; patchelf . args (["--set-interpreter" , dynamic_linker . trim_end ()]) ; } patchelf . arg (fname) ; let _ = patchelf . allow_failure () . run_capture_stdout (exec_ctx) ; }
};
}

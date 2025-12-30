// Generated macro for install_bin (function)
macro_rules! Depcrate_setup_toolchaininstall_bin {
() => {
// Module: crate::setup::toolchain
// Provides: {"install_bin"}
// Dependencies: {}
fn install_bin (bin : & str , dest : & Path , standalone : bool , release : bool) { # [cfg (windows)] use std :: os :: windows :: fs :: symlink_file as symlink ; # [cfg (not (windows))] use std :: os :: unix :: fs :: symlink ; let profile = if release { "release" } else { "debug" } ; let file_name = format ! ("{bin}{EXE_SUFFIX}") ; let mut src = current_dir () . unwrap () ; src . extend (["target" , profile , & file_name]) ; let mut dest = dest . to_path_buf () ; dest . extend (["bin" , & file_name]) ; if standalone { fs :: copy (src , dest) . unwrap () ; } else { symlink (src , dest) . unwrap () ; } }
};
}

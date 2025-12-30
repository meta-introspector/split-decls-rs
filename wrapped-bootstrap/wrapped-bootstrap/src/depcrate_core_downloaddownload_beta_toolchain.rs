// Generated macro for download_beta_toolchain (function)
macro_rules! Depcrate_core_downloaddownload_beta_toolchain {
() => {
// Module: crate::core::download
// Provides: {"download_beta_toolchain"}
// Dependencies: {}
# [cfg (not (test))] pub (crate) fn download_beta_toolchain < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , out : & Path) { let dwn_ctx = dwn_ctx . as_ref () ; dwn_ctx . exec_ctx . verbose (| | { println ! ("downloading stage0 beta artifacts") ; }) ; let date = dwn_ctx . stage0_metadata . compiler . date . clone () ; let version = dwn_ctx . stage0_metadata . compiler . version . clone () ; let extra_components = ["cargo"] ; let sysroot = "stage0" ; download_toolchain (dwn_ctx , out , & version , sysroot , & date , & extra_components , "stage0" , DownloadSource :: Dist ,) ; }
};
}

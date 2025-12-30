// Generated macro for download_file (function)
macro_rules! Depcrate_core_downloaddownload_file {
() => {
// Module: crate::core::download
// Provides: {"download_file"}
// Dependencies: {}
fn download_file < 'a > (dwn_ctx : impl AsRef < DownloadContext < 'a > > , out : & Path , url : & str , dest_path : & Path , help_on_error : & str ,) { let dwn_ctx = dwn_ctx . as_ref () ; dwn_ctx . exec_ctx . verbose (| | { println ! ("download {url}") ; }) ; let tempfile = tempdir (out) . join (dest_path . file_name () . unwrap ()) ; match url . split_once ("://") . map (| (proto , _) | proto) { Some ("http") | Some ("https") => download_http_with_retries (dwn_ctx . host_target , dwn_ctx . is_running_on_ci , dwn_ctx . exec_ctx , & tempfile , url , help_on_error ,) , Some (other) => panic ! ("unsupported protocol {other} in {url}") , None => panic ! ("no protocol in {url}") , } t ! (move_file (& tempfile , dest_path) , format ! ("failed to rename {tempfile:?} to {dest_path:?}")) ; }
};
}

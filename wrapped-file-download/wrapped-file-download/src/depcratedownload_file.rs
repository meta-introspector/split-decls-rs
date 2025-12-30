// Generated macro for download_file (function)
macro_rules! Depcratedownload_file {
() => {
// Module: crate
// Provides: {"download_file"}
// Dependencies: {}
# [doc = " This callback allows the caller to get notified of the download progress modelled by DownloadProgressRecord"] # [doc = " Return \"true\" to continue the download"] # [doc = " Return \"false\" to abort the download"] pub fn download_file < 'a , 'b > (url : & str , destination_file : & Path , use_progress_bar : bool , progress_notify_callback : & 'a mut DownloadProgressCallbackOption < 'b > ,) -> Result < () , String > { download_file_with_headers :: < & str > (url , destination_file , use_progress_bar , progress_notify_callback , & [] ,) }
};
}

// Generated macro for DownloadProgressCallback (type)
macro_rules! DepcrateDownloadProgressCallback {
() => {
// Module: crate
// Provides: {"DownloadProgressCallback"}
// Dependencies: {}
type DownloadProgressCallback < 'a > = Box < dyn FnMut (& DownloadProgressRecord) -> bool + 'a > ;
};
}

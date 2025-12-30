// Generated macro for Downloads (struct)
macro_rules! DepcrateDownloads {
() => {
// Module: crate
// Provides: {"Downloads"}
// Dependencies: {}
struct Downloads { pending : VecDeque < Download > , in_progress : BTreeMap < WorkerId , DownloadInProgress > , }
};
}

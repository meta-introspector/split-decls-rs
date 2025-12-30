// Generated macro for Event (enum)
macro_rules! DepcrateEvent {
() => {
// Module: crate
// Provides: {"Event"}
// Dependencies: {}
enum Event { Input (event :: KeyEvent) , Tick , Resize , DownloadUpdate (WorkerId , DownloadId , f64) , DownloadDone (WorkerId , DownloadId) , }
};
}

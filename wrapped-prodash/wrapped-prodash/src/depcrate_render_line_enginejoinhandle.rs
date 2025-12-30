// Generated macro for JoinHandle (struct)
macro_rules! Depcrate_render_line_engineJoinHandle {
() => {
// Module: crate::render::line::engine
// Provides: {"JoinHandle"}
// Dependencies: {}
# [doc = " A handle to the render thread, which when dropped will instruct it to stop showing progress."] pub struct JoinHandle { inner : Option < std :: thread :: JoinHandle < io :: Result < () > > > , connection : std :: sync :: mpsc :: SyncSender < Event > , disconnected : bool , }
};
}

// Generated macro for DropWakerAction (enum)
macro_rules! Depcrate_headerDropWakerAction {
() => {
// Module: crate::header
// Provides: {"DropWakerAction"}
// Dependencies: {}
# [doc = " Actions to take upon calling [`Header::drop_waker`]."] pub (crate) enum DropWakerAction { # [doc = " Re-schedule the task."] Schedule , # [doc = " Destroy the task."] Destroy , # [doc = " Do nothing."] None , }
};
}

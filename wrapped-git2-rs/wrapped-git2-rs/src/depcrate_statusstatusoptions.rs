// Generated macro for StatusOptions (struct)
macro_rules! Depcrate_statusStatusOptions {
() => {
// Module: crate::status
// Provides: {"StatusOptions"}
// Dependencies: {}
# [doc = " Options that can be provided to `repo.statuses()` to control how the status"] # [doc = " information is gathered."] pub struct StatusOptions { raw : raw :: git_status_options , pathspec : Vec < CString > , ptrs : Vec < * const c_char > , }
};
}

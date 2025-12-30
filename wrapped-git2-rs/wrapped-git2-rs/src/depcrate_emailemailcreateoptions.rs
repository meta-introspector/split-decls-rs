// Generated macro for EmailCreateOptions (struct)
macro_rules! Depcrate_emailEmailCreateOptions {
() => {
// Module: crate::email
// Provides: {"EmailCreateOptions"}
// Dependencies: {}
# [doc = " Options for controlling the formatting of the generated e-mail."] pub struct EmailCreateOptions { diff_options : DiffOptions , diff_find_options : DiffFindOptions , subject_prefix : Option < CString > , raw : raw :: git_email_create_options , }
};
}

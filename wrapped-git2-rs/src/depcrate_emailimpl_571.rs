// Generated macro for impl_571 (impl)
macro_rules! Depcrate_emailimpl_571 {
() => {
// Module: crate::email
// Provides: {"impl_571"}
// Dependencies: {}
impl Default for EmailCreateOptions { fn default () -> Self { let default_options = raw :: git_email_create_options { version : raw :: GIT_EMAIL_CREATE_OPTIONS_VERSION , flags : raw :: GIT_EMAIL_CREATE_DEFAULT as u32 , diff_opts : unsafe { mem :: zeroed () } , diff_find_opts : unsafe { mem :: zeroed () } , subject_prefix : ptr :: null () , start_number : 1 , reroll_number : 0 , } ; let mut diff_options = DiffOptions :: new () ; diff_options . show_binary (true) . context_lines (3) ; Self { diff_options , diff_find_options : DiffFindOptions :: new () , subject_prefix : None , raw : default_options , } } }
};
}

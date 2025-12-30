// Generated macro for RepositoryInitOptions (struct)
macro_rules! Depcrate_repoRepositoryInitOptions {
() => {
// Module: crate::repo
// Provides: {"RepositoryInitOptions"}
// Dependencies: {}
# [doc = " Options which can be used to configure how a repository is initialized"] pub struct RepositoryInitOptions { flags : u32 , mode : u32 , workdir_path : Option < CString > , description : Option < CString > , template_path : Option < CString > , initial_head : Option < CString > , origin_url : Option < CString > , }
};
}

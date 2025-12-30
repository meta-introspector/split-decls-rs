// Generated macro for impl_374 (impl)
macro_rules! Depcrate_options_viewimpl_374 {
() => {
// Module: crate::options::view
// Provides: {"impl_374"}
// Dependencies: {}
impl Columns { fn deduce < V : Vars > (matches : & MatchedFlags < '_ > , vars : & V) -> Result < Self , OptionsError > { let time_types = TimeTypes :: deduce (matches) ? ; let no_git_env = vars . get_with_fallback (vars :: EXA_OVERRIDE_GIT , vars :: EZA_OVERRIDE_GIT) . is_some () ; let git = matches . has (& flags :: GIT) ? && ! matches . has (& flags :: NO_GIT) ? && ! no_git_env ; let subdir_git_repos = matches . has (& flags :: GIT_REPOS) ? && ! matches . has (& flags :: NO_GIT) ? && ! no_git_env ; let subdir_git_repos_no_stat = ! subdir_git_repos && matches . has (& flags :: GIT_REPOS_NO_STAT) ? && ! matches . has (& flags :: NO_GIT) ? && ! no_git_env ; let blocksize = matches . has (& flags :: BLOCKSIZE) ? ; let group = matches . has (& flags :: GROUP) ? ; let inode = matches . has (& flags :: INODE) ? ; let links = matches . has (& flags :: LINKS) ? ; let octal = matches . has (& flags :: OCTAL) ? ; let security_context = xattr :: ENABLED && matches . has (& flags :: SECURITY_CONTEXT) ? ; let file_flags = matches . has (& flags :: FILE_FLAGS) ? ; let permissions = ! matches . has (& flags :: NO_PERMISSIONS) ? ; let filesize = ! matches . has (& flags :: NO_FILESIZE) ? ; let user = ! matches . has (& flags :: NO_USER) ? ; Ok (Self { time_types , inode , links , blocksize , group , git , subdir_git_repos , subdir_git_repos_no_stat , octal , security_context , file_flags , permissions , filesize , user , }) } }
};
}

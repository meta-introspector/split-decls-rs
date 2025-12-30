// Generated macro for UntrackedCache (struct)
macro_rules! Depcrate_extensionUntrackedCache {
() => {
// Module: crate::extension
// Provides: {"UntrackedCache"}
// Dependencies: {}
# [doc = " The extension for untracked files."] # [allow (dead_code)] # [derive (Clone)] pub struct UntrackedCache { # [doc = " Something identifying the location and machine that this cache is for."] # [doc = " Should the repository be copied to a different machine, the entire cache can immediately be invalidated."] identifier : BString , # [doc = " Stat for the .git/info/exclude file"] info_exclude : Option < untracked_cache :: OidStat > , # [doc = " Stat for the `core.excludesfile`"] excludes_file : Option < untracked_cache :: OidStat > , # [doc = " Usually `.gitignore`"] exclude_filename_per_dir : BString , dir_flags : u32 , # [doc = " A list of directories and sub-directories, with `directories[0]` being the root."] directories : Vec < untracked_cache :: Directory > , }
};
}

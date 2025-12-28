macro_rules! deps {
    () => {
        Options!();
        Default!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        # [doc = " Construction"] impl Options { pub (crate) fn from_fs_caps (caps : gix_fs :: Capabilities) -> Self { Self { precompose_unicode : caps . precompose_unicode , ignore_case : caps . ignore_case , recurse_repositories : false , emit_pruned : false , emit_ignored : None , for_deletion : None , emit_tracked : false , emit_untracked : Default :: default () , emit_empty_directories : false , classify_untracked_bare_repositories : false , emit_collapsed : None , empty_patterns_match_prefix : false , symlinks_to_directories_are_ignored_like_directories : false , } } }
    };
}

impl_107!();
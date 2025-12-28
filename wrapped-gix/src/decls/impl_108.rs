macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl From < Options > for gix_dir :: walk :: Options < 'static > { fn from (v : Options) -> Self { gix_dir :: walk :: Options { precompose_unicode : v . precompose_unicode , ignore_case : v . ignore_case , recurse_repositories : v . recurse_repositories , emit_pruned : v . emit_pruned , emit_ignored : v . emit_ignored , for_deletion : v . for_deletion , emit_tracked : v . emit_tracked , emit_untracked : v . emit_untracked , emit_empty_directories : v . emit_empty_directories , classify_untracked_bare_repositories : v . classify_untracked_bare_repositories , emit_collapsed : v . emit_collapsed , symlinks_to_directories_are_ignored_like_directories : v . symlinks_to_directories_are_ignored_like_directories , worktree_relative_worktree_dirs : None , } } }
    };
}

impl_108!()
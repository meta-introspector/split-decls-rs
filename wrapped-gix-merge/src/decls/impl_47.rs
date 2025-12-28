macro_rules! deps {
    () => {
        Options!();
        Pipeline!();
        WorktreeRoots!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Pipeline { # [doc = " Create a new instance of a pipeline which produces blobs suitable for merging."] # [doc = ""] # [doc = " `roots` allow to read worktree files directly, and `worktree_filter` is used"] # [doc = " to transform object database data directly."] # [doc = " `options` are used to further configure the way we act."] pub fn new (roots : WorktreeRoots , worktree_filter : gix_filter :: Pipeline , options : Options) -> Self { Pipeline { roots , filter : worktree_filter , options , path : Default :: default () , } } }
    };
}

impl_47!();
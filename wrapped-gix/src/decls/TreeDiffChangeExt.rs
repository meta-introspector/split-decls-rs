macro_rules! deps {
    () => {
        Change!();
        Repository!();
        Extensions!();
        Note!();
    };
}

macro_rules! TreeDiffChangeExt {
    () => {
        deps!();
        # [doc = " Extensions for [Change](gix_diff::tree_with_rewrites::Change)."] # [cfg (feature = "blob-diff")] pub trait TreeDiffChangeExt { # [doc = " Attach [`old_repo`](crate::Repository) and `new_repo` to current instance. It can be detached later with `detach()`."] # [doc = " Note that both repositories are usually the same."] fn attach < 'old , 'new > (& self , old_repo : & 'old crate :: Repository , new_repo : & 'new crate :: Repository ,) -> crate :: object :: tree :: diff :: Change < '_ , 'old , 'new > ; }
    };
}

TreeDiffChangeExt!();
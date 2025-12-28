macro_rules! Worktree {
    () => {
        # [doc = " An owned git worktree"] # [doc = ""] # [doc = " This structure corresponds to a `git_worktree` in libgit2."] pub struct Worktree { raw : * mut raw :: git_worktree , }
    };
}

Worktree!()
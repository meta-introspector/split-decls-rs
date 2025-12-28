macro_rules! Operation {
    () => {
        # [doc = " The kind of operation to apply using a driver"] # [derive (Debug , Copy , Clone)] pub enum Operation { # [doc = " Turn worktree content into content suitable for storage in `git`."] Clean , # [doc = " Turn content stored in `git` to content suitable for the working tree."] Smudge , }
    };
}

Operation!()
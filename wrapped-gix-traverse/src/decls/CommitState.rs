macro_rules! CommitState {
    () => {
        # [derive (Debug , Clone , Copy)] enum CommitState { # [doc = " The commit may be returned, it hasn't been hidden yet."] Interesting , # [doc = " The commit should not be returned."] Hidden , }
    };
}

CommitState!();
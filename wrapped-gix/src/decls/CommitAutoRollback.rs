macro_rules! deps {
    () => {
        Repository!();
        Config!();
    };
}

macro_rules! CommitAutoRollback {
    () => {
        deps!();
        # [doc = " A utility structure created by [`SnapshotMut::commit_auto_rollback()`] that restores the previous configuration on drop."] pub struct CommitAutoRollback < 'repo > { # [doc = " The owning repository."] pub repo : Option < & 'repo mut Repository > , pub (crate) prev_config : crate :: Config , }
    };
}

CommitAutoRollback!();
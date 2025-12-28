macro_rules! CommitTime {
    () => {
        # [doc = " The timestamp for the creation date of a commit in seconds since unix epoch."] type CommitTime = u32 ;
    };
}

CommitTime!();
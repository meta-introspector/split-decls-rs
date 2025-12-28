macro_rules! raw_entry {
    () => {
        # [cfg (feature = "raw-entry")] mod raw_entry ;
    };
}

raw_entry!()
macro_rules! Duration {
    () => {
        # [derive (Debug , Serialize)] struct Duration { secs : u64 , nanos : u32 , }
    };
}

Duration!()
macro_rules! FairTimeout {
    () => {
        struct FairTimeout { timeout : TimeoutInstant , seed : u32 , }
    };
}

FairTimeout!()
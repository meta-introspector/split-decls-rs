macro_rules! Cause {
    () => {
        type Cause = Box < dyn StdError + Send + Sync > ;
    };
}

Cause!();
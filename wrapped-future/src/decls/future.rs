macro_rules! future {
    () => {
        # [cfg (feature = "std")] mod future ;
    };
}

future!();
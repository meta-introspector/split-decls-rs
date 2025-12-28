macro_rules! random {
    () => {
        # [cfg (feature = "std")] # [cfg (feature = "async-await-macro")] mod random ;
    };
}

random!();
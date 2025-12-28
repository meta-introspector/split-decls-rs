macro_rules! testing {
    () => {
        # [cfg (all (test , feature = "alloc"))] mod testing ;
    };
}

testing!();
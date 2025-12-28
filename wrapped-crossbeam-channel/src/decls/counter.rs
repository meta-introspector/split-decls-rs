macro_rules! counter {
    () => {
        # [cfg (feature = "std")] mod counter ;
    };
}

counter!()
macro_rules! testing {
    () => {
        # [cfg (any (test , feature = "testing"))] pub mod testing ;
    };
}

testing!()
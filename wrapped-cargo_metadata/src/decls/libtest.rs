macro_rules! libtest {
    () => {
        # [cfg (feature = "unstable")] pub mod libtest ;
    };
}

libtest!()
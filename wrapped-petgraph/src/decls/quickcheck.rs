macro_rules! quickcheck {
    () => {
        # [cfg (feature = "quickcheck")] mod quickcheck ;
    };
}

quickcheck!();
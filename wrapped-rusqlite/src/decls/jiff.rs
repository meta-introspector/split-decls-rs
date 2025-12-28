macro_rules! jiff {
    () => {
        # [cfg (feature = "jiff")] mod jiff ;
    };
}

jiff!();
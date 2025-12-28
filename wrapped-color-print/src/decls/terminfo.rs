macro_rules! terminfo {
    () => {
        # [cfg (feature = "terminfo")] mod terminfo ;
    };
}

terminfo!()
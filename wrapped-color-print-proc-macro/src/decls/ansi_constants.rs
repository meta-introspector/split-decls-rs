macro_rules! ansi_constants {
    () => {
        # [cfg (not (feature = "terminfo"))] mod ansi_constants ;
    };
}

ansi_constants!();
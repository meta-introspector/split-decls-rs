macro_rules! ansi {
    () => {
        # [cfg (not (feature = "terminfo"))] mod ansi ;
    };
}

ansi!();
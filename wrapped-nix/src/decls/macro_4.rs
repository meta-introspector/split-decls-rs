macro_rules! macro_4 {
    () => {
        # [cfg (not (target_os = "redox"))] feature ! { #! [feature = "dir"] pub mod dir ; }
    };
}

macro_4!()
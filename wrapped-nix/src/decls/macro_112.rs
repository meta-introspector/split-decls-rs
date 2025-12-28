macro_rules! macro_112 {
    () => {
        # [cfg (not (any (target_os = "redox" , target_os = "fuchsia" , target_os = "solaris" , target_os = "haiku")))] feature ! { #! [feature = "resource"] pub mod resource ; }
    };
}

macro_112!()
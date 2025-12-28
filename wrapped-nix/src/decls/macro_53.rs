macro_rules! macro_53 {
    () => {
        # [cfg (not (any (target_os = "redox" , target_os = "fuchsia")))] feature ! { #! [feature = "term"] # [deny (missing_docs)] pub mod pty ; }
    };
}

macro_53!();
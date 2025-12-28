macro_rules! macro_201 {
    () => {
        # [cfg (all (any (target_os = "freebsd" , solarish , target_os = "linux" , target_os = "netbsd") , feature = "time" , feature = "signal"))] feature ! { #! [feature = "time"] pub mod timer ; }
    };
}

macro_201!()
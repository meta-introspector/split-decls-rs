macro_rules! macro_200 {
    () => {
        # [cfg (linux_android)] feature ! { #! [feature = "time"] pub mod timerfd ; }
    };
}

macro_200!();
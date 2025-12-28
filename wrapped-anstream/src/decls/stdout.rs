macro_rules! deps {
    () => {
        Stdout!();
        AutoStream!();
    };
}

macro_rules! stdout {
    () => {
        deps!();
        # [doc = " Create an ANSI escape code compatible stdout"] # [doc = ""] # [doc = " **Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing"] # [doc = " from the implicit locking in each [`std::io::Write`] call"] # [cfg (feature = "auto")] pub fn stdout () -> Stdout { let stdout = std :: io :: stdout () ; AutoStream :: auto (stdout) }
    };
}

stdout!()
macro_rules! deps {
    () => {
        Stderr!();
        AutoStream!();
    };
}

macro_rules! stderr {
    () => {
        deps!();
        # [doc = " Create an ANSI escape code compatible stderr"] # [doc = ""] # [doc = " **Note:** Call [`AutoStream::lock`] in loops to avoid the performance hit of acquiring/releasing"] # [doc = " from the implicit locking in each [`std::io::Write`] call"] # [cfg (feature = "auto")] pub fn stderr () -> Stderr { let stderr = std :: io :: stderr () ; AutoStream :: auto (stderr) }
    };
}

stderr!();
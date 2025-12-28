macro_rules! com {
    () => {
        # [cfg (all (windows , not (windows_slim_errors)))] mod com ;
    };
}

com!();
macro_rules! WriteMode {
    () => {
        enum WriteMode { Overwrite , Append , }
    };
}

WriteMode!();
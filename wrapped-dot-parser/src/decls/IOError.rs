macro_rules! IOError {
    () => {
        # [doc = " Type for I/O related errors. Those may occur when reading a file for parsing."] pub type IOError = std :: io :: Error ;
    };
}

IOError!();
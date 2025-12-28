macro_rules! PestError {
    () => {
        # [doc = " Type for errors that occur when parsing."] pub type PestError = pest :: error :: Error < Rule > ;
    };
}

PestError!();
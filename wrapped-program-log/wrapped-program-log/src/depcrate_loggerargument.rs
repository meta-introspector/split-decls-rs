// Generated macro for Argument (enum)
macro_rules! Depcrate_loggerArgument {
() => {
// Module: crate::logger
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " Formatting arguments."] # [doc = ""] # [doc = " Arguments can be used to specify additional formatting options for the log message."] # [doc = " Note that types might not support all arguments."] # [non_exhaustive] pub enum Argument { # [doc = " Number of decimal places to display for numbers."] # [doc = ""] # [doc = " This is only applicable for numeric types."] Precision (u8) , # [doc = " Truncate the output at the end when the specified maximum number of characters"] # [doc = " is exceeded."] # [doc = ""] # [doc = " This is only applicable for `str` types."] TruncateEnd (usize) , # [doc = " Truncate the output at the start when the specified maximum number of characters"] # [doc = " is exceeded."] # [doc = ""] # [doc = " This is only applicable for `str` types."] TruncateStart (usize) , }
};
}

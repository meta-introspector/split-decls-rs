// Generated macro for quote_if_needed (function)
macro_rules! Depcratequote_if_needed {
() => {
// Module: crate
// Provides: {"quote_if_needed"}
// Dependencies: {}
# [doc = " Quote an argument that has spaces in it."] # [doc = " When our `WrappedCommand` is printed to the terminal, arguments that contain spaces needed to be quoted."] # [doc = " Otherwise, we will have output such as:"] # [doc = " `pkg-config --libs --cflags foo foo < 3.11`"] # [doc = " which cannot be used in a terminal - it will attempt to read a file named 3.11 and provide it as stdin for pkg-config."] # [doc = " Using this function, we instead get the correct output:"] # [doc = " `pkg-config --libs --cflags foo 'foo < 3.11'`"] fn quote_if_needed (arg : String) -> String { if arg . contains (' ') { format ! ("'{}'" , arg) } else { arg } }
};
}

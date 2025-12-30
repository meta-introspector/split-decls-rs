// Generated macro for ArgSplitFlagValue (struct)
macro_rules! Depcrate_argArgSplitFlagValue {
() => {
// Module: crate::arg
// Provides: {"ArgSplitFlagValue"}
// Dependencies: {}
# [doc = " Yields all values of command line flag `name` as `Ok(arg)`, and all other arguments except"] # [doc = " the flag as `Err(arg)`. (The flag `name` itself is not yielded at all, only its values are.)"] pub struct ArgSplitFlagValue < 'a , I > { args : Option < I > , name : & 'a str , }
};
}

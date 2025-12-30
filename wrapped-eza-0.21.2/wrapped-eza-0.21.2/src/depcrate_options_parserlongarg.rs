// Generated macro for LongArg (type)
macro_rules! Depcrate_options_parserLongArg {
() => {
// Module: crate::options::parser
// Provides: {"LongArg"}
// Dependencies: {}
# [doc = " A **long argument** is a string. This can be a UTF-8 string, even though"] # [doc = " the arguments will all be unchecked `OsString` values, because we don’t"] # [doc = " actually store the user’s input after it’s been matched to a flag, we just"] # [doc = " store which flag it was."] pub type LongArg = & 'static str ;
};
}

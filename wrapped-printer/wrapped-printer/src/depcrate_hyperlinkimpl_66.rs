// Generated macro for impl_66 (impl)
macro_rules! Depcrate_hyperlinkimpl_66 {
() => {
// Module: crate::hyperlink
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > Values < 'a > { # [doc = " Creates a new set of values, starting with the path given."] # [doc = ""] # [doc = " Callers may also set the line and column number using the mutator"] # [doc = " methods."] pub (crate) fn new (path : & 'a HyperlinkPath) -> Values < 'a > { Values { path , line : None , column : None } } # [doc = " Sets the line number for these values."] # [doc = ""] # [doc = " If a line number is not set and a hyperlink format contains a `{line}`"] # [doc = " variable, then it is interpolated with the value of `1` automatically."] pub (crate) fn line (mut self , line : Option < u64 >) -> Values < 'a > { self . line = line ; self } # [doc = " Sets the column number for these values."] # [doc = ""] # [doc = " If a column number is not set and a hyperlink format contains a"] # [doc = " `{column}` variable, then it is interpolated with the value of `1`"] # [doc = " automatically."] pub (crate) fn column (mut self , column : Option < u64 >) -> Values < 'a > { self . column = column ; self } }
};
}

// Generated macro for FormatWith (struct)
macro_rules! Depcrate_formatFormatWith {
() => {
// Module: crate::format
// Provides: {"FormatWith"}
// Dependencies: {}
# [doc = " Format all iterator elements lazily, separated by `sep`."] # [doc = ""] # [doc = " The format value can only be formatted once, after that the iterator is"] # [doc = " exhausted."] # [doc = ""] # [doc = " See [`.format_with()`](crate::Itertools::format_with) for more information."] pub struct FormatWith < 'a , I , F > { sep : & 'a str , # [doc = " `FormatWith` uses interior mutability because `Display::fmt` takes `&self`."] inner : Cell < Option < (I , F) > > , }
};
}

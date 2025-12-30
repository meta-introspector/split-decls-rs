// Generated macro for Format (struct)
macro_rules! Depcrate_formatFormat {
() => {
// Module: crate::format
// Provides: {"Format"}
// Dependencies: {}
# [doc = " Format all iterator elements lazily, separated by `sep`."] # [doc = ""] # [doc = " The format value can only be formatted once, after that the iterator is"] # [doc = " exhausted."] # [doc = ""] # [doc = " See [`.format()`](crate::Itertools::format)"] # [doc = " for more information."] pub struct Format < 'a , I > { sep : & 'a str , # [doc = " `Format` uses interior mutability because `Display::fmt` takes `&self`."] inner : Cell < Option < I > > , }
};
}

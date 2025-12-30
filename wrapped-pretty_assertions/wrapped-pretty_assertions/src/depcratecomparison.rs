// Generated macro for Comparison (struct)
macro_rules! DepcrateComparison {
() => {
// Module: crate
// Provides: {"Comparison"}
// Dependencies: {}
# [doc = " A comparison of two values."] # [doc = ""] # [doc = " Where both values implement `Debug`, the comparison can be displayed as a pretty diff."] # [doc = ""] # [doc = " ```"] # [doc = " use pretty_assertions::Comparison;"] # [doc = ""] # [doc = " print!(\"{}\", Comparison::new(&123, &134));"] # [doc = " ```"] # [doc = ""] # [doc = " The values may have different types, although in practice they are usually the same."] pub struct Comparison < 'a , TLeft , TRight > where TLeft : ? Sized , TRight : ? Sized , { left : & 'a TLeft , right : & 'a TRight , }
};
}

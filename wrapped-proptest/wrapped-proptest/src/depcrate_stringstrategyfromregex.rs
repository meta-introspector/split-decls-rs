// Generated macro for StrategyFromRegex (trait)
macro_rules! Depcrate_stringStrategyFromRegex {
() => {
// Module: crate::string
// Provides: {"StrategyFromRegex"}
// Dependencies: {}
# [doc (hidden)] # [doc = " A type which knows how to produce a `Strategy` from a regular expression"] # [doc = " generating the type."] # [doc = ""] # [doc = " This trait exists for the benefit of `#[proptest(regex = \"...\")]`."] # [doc = " It is semver exempt, so use at your own risk."] # [doc = " If you found a use for the trait beyond `Vec<u8>` and `String`,"] # [doc = " please file an issue at https://github.com/proptest-rs/proptest."] pub trait StrategyFromRegex : Sized + fmt :: Debug { type Strategy : Strategy < Value = Self > ; # [doc = " Produce a strategy for `Self` from the `regex`."] fn from_regex (regex : & str) -> Self :: Strategy ; }
};
}

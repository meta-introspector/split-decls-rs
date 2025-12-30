// Generated macro for impl_72 (impl)
macro_rules! Depcrate_parsingimpl_72 {
() => {
// Module: crate::parsing
// Provides: {"impl_72"}
// Dependencies: {}
impl RawPattern { # [doc = " Returns this search pattern as Rust source code that we can feed to the Rust parser."] fn as_rust_code (& self) -> String { let mut res = String :: new () ; for t in & self . tokens { res . push_str (match t { PatternElement :: Token (token) => token . text . as_str () , PatternElement :: Placeholder (placeholder) => placeholder . stand_in_name . as_str () , }) ; } res } pub (crate) fn placeholders_by_stand_in (& self) -> FxHashMap < SmolStr , Placeholder > { let mut res = FxHashMap :: default () ; for t in & self . tokens { if let PatternElement :: Placeholder (placeholder) = t { res . insert (SmolStr :: new (& placeholder . stand_in_name) , placeholder . clone ()) ; } } res } }
};
}

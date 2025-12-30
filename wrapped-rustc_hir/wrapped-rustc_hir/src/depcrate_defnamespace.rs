// Generated macro for Namespace (enum)
macro_rules! Depcrate_defNamespace {
() => {
// Module: crate::def
// Provides: {"Namespace"}
// Dependencies: {}
# [doc = " Different kinds of symbols can coexist even if they share the same textual name."] # [doc = " Therefore, they each have a separate universe (known as a \"namespace\")."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Encodable , Decodable)] # [derive (HashStable_Generic)] pub enum Namespace { # [doc = " The type namespace includes `struct`s, `enum`s, `union`s, `trait`s, and `mod`s"] # [doc = " (and, by extension, crates)."] # [doc = ""] # [doc = " Note that the type namespace includes other items; this is not an"] # [doc = " exhaustive list."] TypeNS , # [doc = " The value namespace includes `fn`s, `const`s, `static`s, and local variables (including function arguments)."] ValueNS , # [doc = " The macro namespace includes `macro_rules!` macros, declarative `macro`s,"] # [doc = " procedural macros, attribute macros, `derive` macros, and non-macro attributes"] # [doc = " like `#[inline]` and `#[rustfmt::skip]`."] MacroNS , }
};
}

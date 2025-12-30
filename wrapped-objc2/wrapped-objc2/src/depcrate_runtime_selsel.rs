// Generated macro for Sel (struct)
macro_rules! Depcrate_runtime_selSel {
() => {
// Module: crate::runtime::sel
// Provides: {"Sel"}
// Dependencies: {}
# [doc = " A method selector."] # [doc = ""] # [doc = " The Rust equivalent of Objective-C's `SEL _Nonnull` type. You can create"] # [doc = " this statically using the [`sel!`] macro."] # [doc = ""] # [doc = " The main reason the Objective-C runtime uses a custom type for selectors,"] # [doc = " as opposed to a plain c-string, is to support efficient comparison - a"] # [doc = " a selector is effectively an [interned string], so this makes equiality"] # [doc = " comparisons very cheap."] # [doc = ""] # [doc = " This struct guarantees the null-pointer optimization, namely that"] # [doc = " `Option<Sel>` is the same size as `Sel`."] # [doc = ""] # [doc = " Selectors are immutable."] # [doc = ""] # [doc = " [`sel!`]: crate::sel"] # [doc = " [interned string]: https://en.wikipedia.org/wiki/String_interning"] # [repr (transparent)] # [derive (Copy , Clone)] # [doc (alias = "SEL")] # [doc (alias = "objc_selector")] pub struct Sel { ptr : NonNull < c_void > , }
};
}

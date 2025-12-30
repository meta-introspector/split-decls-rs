// Generated macro for macro_7446 (macro)
macro_rules! Depcrate_missing_trait_methodsmacro_7446 {
() => {
// Module: crate::missing_trait_methods
// Provides: {"macro_7446"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if a provided method is used implicitly by a trait"] # [doc = " implementation."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " To ensure that a certain implementation implements every method; for example,"] # [doc = " a wrapper type where every method should delegate to the corresponding method of"] # [doc = " the inner type's implementation."] # [doc = ""] # [doc = " This lint should typically be enabled on a specific trait `impl` item"] # [doc = " rather than globally."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " trait Trait {"] # [doc = "     fn required();"] # [doc = ""] # [doc = "     fn provided() {}"] # [doc = " }"] # [doc = ""] # [doc = " # struct Type;"] # [doc = " #[warn(clippy::missing_trait_methods)]"] # [doc = " impl Trait for Type {"] # [doc = "     fn required() { /* ... */ }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " trait Trait {"] # [doc = "     fn required();"] # [doc = ""] # [doc = "     fn provided() {}"] # [doc = " }"] # [doc = ""] # [doc = " # struct Type;"] # [doc = " #[warn(clippy::missing_trait_methods)]"] # [doc = " impl Trait for Type {"] # [doc = "     fn required() { /* ... */ }"] # [doc = ""] # [doc = "     fn provided() { /* ... */ }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub MISSING_TRAIT_METHODS , restriction , "trait implementation uses default provided method" }
};
}

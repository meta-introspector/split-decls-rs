// Generated macro for macro_10247 (macro)
macro_rules! Depcrate_trait_boundsmacro_10247 {
() => {
// Module: crate::trait_bounds
// Provides: {"macro_10247"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for cases where generics or trait objects are being used and multiple"] # [doc = " syntax specifications for trait bounds are used simultaneously."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Duplicate bounds makes the code"] # [doc = " less readable than specifying them only once."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn func<T: Clone + Default>(arg: T) where T: Clone + Default {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # mod hidden {"] # [doc = " fn func<T: Clone + Default>(arg: T) {}"] # [doc = " # }"] # [doc = ""] # [doc = " // or"] # [doc = ""] # [doc = " fn func<T>(arg: T) where T: Clone + Default {}"] # [doc = " ```"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn foo<T: Default + Default>(bar: T) {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo<T: Default>(bar: T) {}"] # [doc = " ```"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn foo<T>(bar: T) where T: Default + Default {}"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo<T>(bar: T) where T: Default {}"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub TRAIT_DUPLICATION_IN_BOUNDS , nursery , "check if the same trait bounds are specified more than once during a generic declaration" }
};
}

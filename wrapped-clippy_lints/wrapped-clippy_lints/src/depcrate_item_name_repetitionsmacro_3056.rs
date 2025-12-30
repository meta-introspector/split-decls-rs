// Generated macro for macro_3056 (macro)
macro_rules! Depcrate_item_name_repetitionsmacro_3056 {
() => {
// Module: crate::item_name_repetitions
// Provides: {"macro_3056"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects enumeration variants that are prefixed or suffixed"] # [doc = " by the same characters."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Enumeration variant names should specify their variant,"] # [doc = " not repeat the enumeration name."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " Characters with no casing will be considered when comparing prefixes/suffixes"] # [doc = " This applies to numbers and non-ascii characters without casing"] # [doc = " e.g. `Foo1` and `Foo2` is considered to have different prefixes"] # [doc = " (the prefixes are `Foo1` and `Foo2` respectively), as also `Bar螃`, `Bar蟹`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " enum Cake {"] # [doc = "     BlackForestCake,"] # [doc = "     HummingbirdCake,"] # [doc = "     BattenbergCake,"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " enum Cake {"] # [doc = "     BlackForest,"] # [doc = "     Hummingbird,"] # [doc = "     Battenberg,"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ENUM_VARIANT_NAMES , style , "enums where all variants share a prefix/postfix" }
};
}

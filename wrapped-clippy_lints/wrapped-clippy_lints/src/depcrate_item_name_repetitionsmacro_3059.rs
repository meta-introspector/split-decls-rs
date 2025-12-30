// Generated macro for macro_3059 (macro)
macro_rules! Depcrate_item_name_repetitionsmacro_3059 {
() => {
// Module: crate::item_name_repetitions
// Provides: {"macro_3059"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects struct fields that are prefixed or suffixed"] # [doc = " by the same characters or the name of the struct itself."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Information common to all struct fields is better represented in the struct name."] # [doc = ""] # [doc = " ### Limitations"] # [doc = " Characters with no casing will be considered when comparing prefixes/suffixes"] # [doc = " This applies to numbers and non-ascii characters without casing"] # [doc = " e.g. `foo1` and `foo2` is considered to have different prefixes"] # [doc = " (the prefixes are `foo1` and `foo2` respectively), as also `bar螃`, `bar蟹`"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct Cake {"] # [doc = "     cake_sugar: u8,"] # [doc = "     cake_flour: u8,"] # [doc = "     cake_eggs: u8"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " struct Cake {"] # [doc = "     sugar: u8,"] # [doc = "     flour: u8,"] # [doc = "     eggs: u8"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.75.0"] pub STRUCT_FIELD_NAMES , pedantic , "structs where all fields share a prefix/postfix or contain the name of the struct" }
};
}

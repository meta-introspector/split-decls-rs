// Generated macro for macro_1062 (macro)
macro_rules! Depcrate_nonstandard_stylemacro_1062 {
() => {
// Module: crate::nonstandard_style
// Provides: {"macro_1062"}
// Dependencies: {}
declare_lint ! { # [doc = " The `non_camel_case_types` lint detects types, variants, traits and"] # [doc = " type parameters that don't have camel case names."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " struct my_struct;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The preferred style for these identifiers is to use \"camel case\", such"] # [doc = " as `MyStruct`, where the first letter should not be lowercase, and"] # [doc = " should not use underscores between letters. Underscores are allowed at"] # [doc = " the beginning and end of the identifier, as well as between"] # [doc = " non-letters (such as `X86_64`)."] pub NON_CAMEL_CASE_TYPES , Warn , "types, variants, traits and type parameters should have camel case names" }
};
}

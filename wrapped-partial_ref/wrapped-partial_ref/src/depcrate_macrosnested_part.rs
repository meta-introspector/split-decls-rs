// Generated macro for nested_part (macro)
macro_rules! Depcrate_macrosnested_part {
() => {
// Module: crate::macros
// Provides: {"nested_part"}
// Dependencies: {}
# [doc = " Expands `A | B | ... | Z` to `Nested<...Nested<A, B>, ..., Z>`"] # [doc = ""] # [doc = " This mirrors the syntax for the expressions that can be passed to [`PartialRef`]'s part"] # [doc = " functions."] # [macro_export] macro_rules ! nested_part { ($ target : ty) => { $ target } ; ($ target : ty | $ part : ty) => { $ crate :: Nested <$ target , $ part > } ; ($ target : ty | $ part : ty | $ ($ subparts : ty) |*) => { nested_part ! ($ crate :: Nested <$ target , $ part > | $ ($ subparts) |*) } ; }
};
}

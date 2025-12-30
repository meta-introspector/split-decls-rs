// Generated macro for ElementLocation (enum)
macro_rules! Depcrate_transliterate_compile_parseElementLocation {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"ElementLocation"}
// Dependencies: {}
# [doc = " The location in which an element can appear. Used for error reporting in [`CompileError`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum ElementLocation { # [doc = " The element appears on the source side of a rule (i.e., the side _not_ pointed at"] # [doc = " by the arrow)."] Source , # [doc = " The element appears on the target side of a rule (i.e., the side pointed at by the arrow)."] Target , # [doc = " The element appears inside a variable definition."] VariableDefinition , }
};
}

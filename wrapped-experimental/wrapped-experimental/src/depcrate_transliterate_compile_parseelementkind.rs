// Generated macro for ElementKind (enum)
macro_rules! Depcrate_transliterate_compile_parseElementKind {
() => {
// Module: crate::transliterate::compile::parse
// Provides: {"ElementKind"}
// Dependencies: {}
# [doc = " An element that can appear in a rule. Used for error reporting in [`CompileError`]."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum ElementKind { # [doc = " A literal string: `abc 'abc'`."] Literal , # [doc = " A variable reference: `$var`."] VariableReference , # [doc = " A backreference to a segment: `$1`."] BackReference , # [doc = " A quantifier of any sort: `c*`, `c+`, `c?`."] Quantifier , # [doc = " A segment: `(abc)`."] Segment , # [doc = " A UnicodeSet: `[a-z]`."] UnicodeSet , # [doc = " A function call: `&[a-z] Remove(...)`."] FunctionCall , # [doc = " A cursor: `|`."] Cursor , # [doc = " A start anchor: `^`."] AnchorStart , # [doc = " An end anchor: `$`."] AnchorEnd , }
};
}

// Generated macro for impl_3471 (impl)
macro_rules! Depcrate_literal_representationimpl_3471 {
() => {
// Module: crate::literal_representation
// Provides: {"impl_3471"}
// Dependencies: {}
impl WarningType { fn lint_and_text (& self) -> (& 'static Lint , & 'static str , & 'static str) { match self { Self :: MistypedLiteralSuffix => (MISTYPED_LITERAL_SUFFIXES , "mistyped literal suffix" , "did you mean to write" ,) , Self :: UnreadableLiteral => (UNREADABLE_LITERAL , "long literal lacking separators" , "consider") , Self :: LargeDigitGroups => (LARGE_DIGIT_GROUPS , "digit groups should be smaller" , "consider") , Self :: InconsistentDigitGrouping => (INCONSISTENT_DIGIT_GROUPING , "digits grouped inconsistently by underscores" , "consider" ,) , Self :: DecimalRepresentation => (DECIMAL_LITERAL_REPRESENTATION , "integer literal has a better hexadecimal representation" , "consider" ,) , Self :: UnusualByteGroupings => (UNUSUAL_BYTE_GROUPINGS , "digits of hex, binary or octal literal not in groups of equal size" , "consider" ,) , } } fn display (& self , num_lit : & NumericLiteral < '_ > , cx : & EarlyContext < '_ > , span : Span) { let (lint , message , try_msg) = self . lint_and_text () ; # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , lint , span , message , | diag | { diag . span_suggestion (span , try_msg , num_lit . format () , Applicability :: MaybeIncorrect) ; }) ; } }
};
}

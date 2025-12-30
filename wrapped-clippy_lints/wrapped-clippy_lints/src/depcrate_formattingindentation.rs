// Generated macro for indentation (function)
macro_rules! Depcrate_formattingindentation {
() => {
// Module: crate::formatting
// Provides: {"indentation"}
// Dependencies: {}
fn indentation (cx : & EarlyContext < '_ > , span : Span) -> usize { cx . sess () . source_map () . lookup_char_pos (span . lo ()) . col . 0 }
};
}

// Generated macro for suggest_size_hint (function)
macro_rules! Depcrate_parser_repeatsuggest_size_hint {
() => {
// Module: crate::parser::repeat
// Provides: {"suggest_size_hint"}
// Dependencies: {}
fn suggest_size_hint < I > (iterator : I , (min , max) : (usize , Option < usize >)) -> SuggestSizeHint < I > where I : Iterator , { SuggestSizeHint { iterator , min : cmp :: min (min , 4096) , max , } }
};
}

// Generated macro for normalize_whitespace (function)
macro_rules! Depcrate_renderer_rendernormalize_whitespace {
() => {
// Module: crate::renderer::render
// Provides: {"normalize_whitespace"}
// Dependencies: {}
pub (crate) fn normalize_whitespace (s : & str) -> String { s . chars () . fold (String :: with_capacity (s . len ()) , | mut s , c | { match OUTPUT_REPLACEMENTS . binary_search_by_key (& c , | (k , _) | * k) { Ok (i) => s . push_str (OUTPUT_REPLACEMENTS [i] . 1) , _ => s . push (c) , } s }) }
};
}

// Generated macro for math_to_emoji_impl (function)
macro_rules! Depcrate_emoji_poetrymath_to_emoji_impl {
() => {
// Module: crate::emoji_poetry
// Provides: {"math_to_emoji_impl"}
// Dependencies: {}
# [decl (fn , name = "math_to_emoji_impl" , vis = "pub" , hash = "d9c2ffb8")] pub fn math_to_emoji_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let math_expr = input_str . value () ; quote ! { { println ! ("cargo:warning=🧮 Converting math to emoji: {}" , # math_expr) ; let emoji_math = # math_expr . replace ("Rustc" , "🦀") . replace ("Monster" , "👹") . replace ("DAO" , "🏛️") . replace ("→" , "→") . replace ("∞" , "∞") . replace ("∀" , "∀") . replace ("∃" , "∃") . replace ("∈" , "∈") . replace ("⊆" , "⊆") . replace ("∪" , "∪") . replace ("∩" , "∩") . replace ("≅" , "≅") . replace ("≡" , "≡") . replace ("⟨" , "⟨") . replace ("⟩" , "⟩") . replace ("φ" , "φ") . replace ("L(s)" , "∞(s)") . replace ("proof" , "✅") . replace ("theorem" , "📐") . replace ("definition" , "📝") . replace ("lemma" , "💎") . replace ("compile_error" , "❌") . replace ("verify" , "🔍") . replace ("true" , "✅") . replace ("false" , "❌") ; format ! ("🧮 {} 🎭" , emoji_math) } } . into () }
};
}

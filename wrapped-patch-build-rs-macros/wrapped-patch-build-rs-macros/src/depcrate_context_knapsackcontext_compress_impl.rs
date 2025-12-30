// Generated macro for context_compress_impl (function)
macro_rules! Depcrate_context_knapsackcontext_compress_impl {
() => {
// Module: crate::context_knapsack
// Provides: {"context_compress_impl"}
// Dependencies: {}
# [decl (fn , name = "context_compress_impl" , vis = "pub" , hash = "7e09cb3e")] pub fn context_compress_impl (input : TokenStream) -> TokenStream { let input_str = parse_macro_input ! (input as LitStr) ; let full_context = input_str . value () ; quote ! { { println ! ("cargo:warning=🗜️ Compressing context for optimal packing") ; let compressed = # full_context . lines () . filter (| line | ! line . trim () . is_empty ()) . filter (| line | ! line . trim () . starts_with ("//")) . map (| line | line . trim ()) . filter (| line | line . len () > 3) . collect ::< Vec < _ >> () . join (" ") ; let ultra_compressed = compressed . replace ("  " , " ") . replace (" { " , "{") . replace (" } " , "}") . replace (" ( " , "(") . replace (" ) " , ")") ; let compression_ratio = # full_context . len () as f64 / ultra_compressed . len () as f64 ; println ! ("cargo:warning=📉 Compression: {:.1}x ratio" , compression_ratio) ; ultra_compressed } } . into () }
};
}

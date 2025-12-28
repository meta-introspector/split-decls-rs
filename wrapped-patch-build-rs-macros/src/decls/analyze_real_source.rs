macro_rules! analyze_real_source {
    () => {
        # [proc_macro] # [decl2 (fn , name = "analyze_real_source" , vis = "pub" , hash = "45d79d1d")] pub fn analyze_real_source (input : TokenStream) -> TokenStream { real_rustc_analysis :: analyze_real_source_impl (input) }
    };
}

analyze_real_source!();
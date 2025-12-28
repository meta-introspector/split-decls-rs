macro_rules! ollama {
    () => {
        # [proc_macro] # [decl2 (fn , name = "ollama" , vis = "pub" , hash = "9c48b75e")] pub fn ollama (input : TokenStream) -> TokenStream { ollama_macros :: ollama_impl (input) }
    };
}

ollama!()
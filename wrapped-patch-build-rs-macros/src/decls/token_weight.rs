macro_rules! token_weight {
    () => {
        # [proc_macro] # [decl2 (fn , name = "token" , vis = "pub" , hash = "280ff363")] pub fn token_weight (input : TokenStream) -> TokenStream { context_knapsack :: token_weight_impl (input) }
    };
}

token_weight!();
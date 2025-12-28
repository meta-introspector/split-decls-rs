macro_rules! lean4_expr_json {
    () => {
        # [proc_macro] # [decl2 (fn , name = "lean4_expr_json" , vis = "pub" , hash = "086ed9c7")] pub fn lean4_expr_json (input : TokenStream) -> TokenStream { lean4_json :: lean4_expr_json_impl (input) }
    };
}

lean4_expr_json!();
macro_rules! math_to_emoji {
    () => {
        # [proc_macro] # [decl2 (fn , name = "math_to_emoji" , vis = "pub" , hash = "36cce0e6")] pub fn math_to_emoji (input : TokenStream) -> TokenStream { emoji_poetry :: math_to_emoji_impl (input) }
    };
}

math_to_emoji!()
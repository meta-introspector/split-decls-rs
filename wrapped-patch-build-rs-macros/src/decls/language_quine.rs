macro_rules! language_quine {
    () => {
        # [proc_macro] # [decl2 (fn , name = "language_quine" , vis = "pub" , hash = "2ba57705")] pub fn language_quine (input : TokenStream) -> TokenStream { quine_relay :: language_quine_impl (input) }
    };
}

language_quine!()
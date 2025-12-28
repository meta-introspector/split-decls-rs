macro_rules! checktemplate {
    () => {
        # [proc_macro] # [decl2 (fn , name = "checktemplate" , vis = "pub" , hash = "0ddf638b")] pub fn checktemplate (input : TokenStream) -> TokenStream { template_checker :: checktemplate_impl (input) }
    };
}

checktemplate!()
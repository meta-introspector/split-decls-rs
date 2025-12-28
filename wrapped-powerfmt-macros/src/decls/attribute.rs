macro_rules! attribute {
    () => {
        # [doc = " Declare an attribute macro."] macro_rules ! attribute { ($ ($ mod : ident) ::+ => $ public : ident) => { # [doc = concat ! ("The `#[" , attribute ! (@ stringify $ ($ mod) +) , "]` attribute.")] # [proc_macro_attribute] pub fn $ public (attr : TokenStream , item : TokenStream) -> TokenStream { match $ ($ mod) ::+:: implementation (attr . into () , item . into ()) { Ok (ts) => ts . into () , Err (err) => err . to_compile_error () . into () , } } } ; (@ stringify $ first : ident $ ($ remaining : ident) *) => { concat ! (stringify ! ($ first) , $ ("::" , stringify ! ($ remaining) ,) *) } ; }
    };
}

attribute!();
macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! format_token {
    () => {
        deps!();
        # [doc = " Formatting macro for constructing a `TokenStream`."] # [doc = ""] # [doc = " <br>"] # [doc = ""] # [doc = " # Syntax"] # [doc = ""] # [doc = " Syntax is copied from the [`format!`] macro, supporting both positional and"] # [doc = " named arguments."] # [macro_export] # [doc (hidden)] macro_rules ! format_token { ($ ($ fmt : tt) *) => { $ crate :: TokenStream :: from (format ! ($ ($ fmt) *)) } ; }
    };
}

format_token!()
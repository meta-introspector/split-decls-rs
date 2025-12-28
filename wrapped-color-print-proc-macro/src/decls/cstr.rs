macro_rules! cstr {
    () => {
        # [doc = " Colorizes a string literal, without formatting the `format!`-like placeholders."] # [doc = ""] # [doc = " * Accepts only one argument;"] # [doc = " * Will panic if feature `terminfo` is activated."] # [cfg (feature = "terminfo")] # [proc_macro] pub fn cstr (_ : TokenStream) -> TokenStream { panic ! ("Macro cstr!() cannot be used with terminfo feature") }
    };
}

cstr!();
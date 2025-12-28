macro_rules! untagged {
    () => {
        # [doc = " Removes all the color tags from the given string literal."] # [doc = ""] # [doc = " Accepts only one argument."] # [doc = ""] # [doc = " #### Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use color_print_proc_macro::untagged;"] # [doc = " let s: &str = untagged!(\"A <g>normal</> word\");"] # [doc = " assert_eq!(s, \"A normal word\");"] # [doc = " ```"] # [proc_macro] pub fn untagged (input : TokenStream) -> TokenStream { crate :: untagged :: get_untagged (input) . unwrap_or_else (| err | err . to_token_stream ()) . into () }
    };
}

untagged!();
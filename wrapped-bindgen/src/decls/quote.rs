macro_rules! deps {
    () => {
        TokenStream!();
        ToTokens!();
    };
}

macro_rules! quote {
    () => {
        deps!();
        # [doc = " The whole point."] # [doc = ""] # [doc = " Performs variable interpolation against the input and produces it as"] # [doc = " [`TokenStream`]."] # [doc = ""] # [doc = " # Interpolation"] # [doc = ""] # [doc = " Variable interpolation is done with `#var` (similar to `$var` in"] # [doc = " `macro_rules!` macros). This grabs the `var` variable that is currently in"] # [doc = " scope and inserts it in that location in the output tokens. Any type"] # [doc = " implementing the [`ToTokens`] trait can be interpolated. This includes most"] # [doc = " Rust primitive types."] # [doc = ""] # [doc = " [`ToTokens`]: trait.ToTokens.html"] # [doc = ""] # [doc = " Repetition is done using `#(...)*` or `#(...),*` again similar to"] # [doc = " `macro_rules!`. This iterates through the elements of any variable"] # [doc = " interpolated within the repetition and inserts a copy of the repetition body"] # [doc = " for each one. The variables in an interpolation may be a `Vec`, slice,"] # [doc = " `BTreeSet`, or any `Iterator`."] # [doc = ""] # [doc = " - `#(#var)*` — no separators"] # [doc = " - `#(#var),*` — the character before the asterisk is used as a separator"] # [doc = " - `#( struct #var; )*` — the repetition can contain other tokens"] # [doc = " - `#( #k => println!(\"{}\", #v), )*` — even multiple interpolations"] # [macro_export] # [doc (hidden)] macro_rules ! quote { () => { $ crate :: tokens :: TokenStream :: new () } ; ($ ($ tt : tt) *) => { { let mut _s = $ crate :: tokens :: TokenStream :: new () ; $ crate :: quote_each_token ! (_s $ ($ tt) *) ; _s } } ; }
    };
}

quote!();
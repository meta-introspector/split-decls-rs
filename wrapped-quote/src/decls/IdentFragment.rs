macro_rules! IdentFragment {
    () => {
        # [doc = " Specialized formatting trait used by `format_ident!`."] # [doc = ""] # [doc = " [`Ident`] arguments formatted using this trait will have their `r#` prefix"] # [doc = " stripped, if present."] # [doc = ""] # [doc = " See [`format_ident!`] for more information."] # [doc = ""] # [doc = " [`format_ident!`]: crate::format_ident"] pub trait IdentFragment { # [doc = " Format this value as an identifier fragment."] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result ; # [doc = " Span associated with this `IdentFragment`."] # [doc = ""] # [doc = " If non-`None`, may be inherited by formatted identifiers."] fn span (& self) -> Option < Span > { None } }
    };
}

IdentFragment!()
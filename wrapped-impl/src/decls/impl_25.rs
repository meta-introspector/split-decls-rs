macro_rules! deps {
    () => {
        Display!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl ToTokens for Display < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { if self . infinite_recursive { let span = self . fmt . span () ; tokens . extend (quote_spanned ! { span => # [warn (unconditional_recursion)] fn _fmt () { _fmt () } }) ; } let fmt = & self . fmt ; let args = & self . args ; let write = if self . requires_fmt_machinery { quote ! { :: core :: write ! (__formatter , # fmt # args) } } else { quote ! { __formatter . write_str (# fmt) } } ; tokens . extend (if self . bindings . is_empty () { write } else { let locals = self . bindings . iter () . map (| (local , _value) | local) ; let values = self . bindings . iter () . map (| (_local , value) | value) ; quote ! { match (# (# values ,) *) { (# (# locals ,) *) => # write } } }) ; } }
    };
}

impl_25!();
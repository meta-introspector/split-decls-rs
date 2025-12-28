macro_rules! deps {
    () => {
        SnippetCap!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl SnippetCap { pub const fn new (allow_snippets : bool) -> Option < SnippetCap > { if allow_snippets { Some (SnippetCap { _private : () }) } else { None } } }
    };
}

impl_43!()
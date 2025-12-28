macro_rules! deps {
    () => {
        SnippetCap!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl SnippetCap { pub const fn new (allow_snippets : bool) -> Option < SnippetCap > { if allow_snippets { Some (SnippetCap { _private : () }) } else { None } } }
    };
}

impl_261!();
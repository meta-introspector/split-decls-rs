macro_rules! deps {
    () => {
        Suggestions!();
        CodeSuggestion!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Suggestions { # [doc = " Returns the underlying list of suggestions."] pub fn unwrap_tag (self) -> Vec < CodeSuggestion > { match self { Suggestions :: Enabled (suggestions) => suggestions , Suggestions :: Sealed (suggestions) => suggestions . into_vec () , Suggestions :: Disabled => Vec :: new () , } } }
    };
}

impl_25!()
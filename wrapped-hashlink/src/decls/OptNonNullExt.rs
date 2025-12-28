macro_rules! OptNonNullExt {
    () => {
        trait OptNonNullExt < T > { # [allow (clippy :: wrong_self_convention)] fn as_ptr (self) -> * mut T ; }
    };
}

OptNonNullExt!()
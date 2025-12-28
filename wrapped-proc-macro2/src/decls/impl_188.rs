macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        # [doc = " `TokenStream::default()` returns an empty stream,"] # [doc = " i.e. this is equivalent with `TokenStream::new()`."] impl Default for TokenStream { fn default () -> Self { TokenStream :: new () } }
    };
}

impl_188!();
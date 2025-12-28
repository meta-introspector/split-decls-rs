macro_rules! deps {
    () => {
        ContextSize!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        # [doc = " Instantiation"] impl ContextSize { # [doc = " Create a symmetrical context with `n` lines before and after a changed hunk."] pub fn symmetrical (n : u32) -> Self { ContextSize { symmetrical : n } } }
    };
}

impl_128!();
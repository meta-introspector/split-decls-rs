macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl From < proc_macro :: Span > for Span { fn from (src : proc_macro :: Span) -> Self { Self :: One (src) } }
    };
}

impl_107!();
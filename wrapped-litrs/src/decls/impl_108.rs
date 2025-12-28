macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [cfg (feature = "proc-macro2")] impl From < proc_macro2 :: Span > for Span { fn from (src : proc_macro2 :: Span) -> Self { Self :: Two (src) } }
    };
}

impl_108!()
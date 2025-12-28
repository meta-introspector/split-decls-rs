macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Span { # [doc = " Execute `f` in with this span active, consuming it."] pub fn into_scope < T > (self , f : impl FnOnce () -> T) -> T { f () } }
    };
}

impl_14!();
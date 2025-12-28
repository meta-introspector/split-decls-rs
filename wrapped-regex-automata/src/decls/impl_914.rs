macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_914 {
    () => {
        deps!();
        impl From < Span > for Range < usize > { # [inline] fn from (span : Span) -> Range < usize > { Range { start : span . start , end : span . end } } }
    };
}

impl_914!()
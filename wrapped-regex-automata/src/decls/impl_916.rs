macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_916 {
    () => {
        deps!();
        impl PartialEq < Span > for Range < usize > { # [inline] fn eq (& self , span : & Span) -> bool { self . start == span . start && self . end == span . end } }
    };
}

impl_916!();
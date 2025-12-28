macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_915 {
    () => {
        deps!();
        impl PartialEq < Range < usize > > for Span { # [inline] fn eq (& self , range : & Range < usize >) -> bool { self . start == range . start && self . end == range . end } }
    };
}

impl_915!()
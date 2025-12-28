macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_230 {
    () => {
        deps!();
        impl Ord for Ident { fn cmp (& self , other : & Ident) -> Ordering { self . to_string () . cmp (& other . to_string ()) } }
    };
}

impl_230!();
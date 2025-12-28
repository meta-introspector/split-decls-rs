macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl PartialOrd for Ident { fn partial_cmp (& self , other : & Ident) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_59!()
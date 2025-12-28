macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl From < Ident > for Sp < String > { fn from (ident : Ident) -> Self { Sp { val : ident . to_string () , span : ident . span () , } } }
    };
}

impl_95!()
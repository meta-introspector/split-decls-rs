macro_rules! deps {
    () => {
        Sp!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl From < LitStr > for Sp < String > { fn from (lit : LitStr) -> Self { Sp { val : lit . value () , span : lit . span () , } } }
    };
}

impl_96!();
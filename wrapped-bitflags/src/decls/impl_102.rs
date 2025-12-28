macro_rules! deps {
    () => {
        Flag!();
        Iter!();
        IterNames!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < B : 'static > Iter < B > { # [doc (hidden)] pub const fn __private_const_new (flags : & 'static [Flag < B >] , source : B , remaining : B) -> Self { Iter { inner : IterNames :: __private_const_new (flags , source , remaining) , done : false , } } }
    };
}

impl_102!()
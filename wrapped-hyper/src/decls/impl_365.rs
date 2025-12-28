macro_rules! impl_365 {
    () => {
        impl hyper_context < '_ > { pub (crate) fn wrap < 'a , 'b > (cx : & 'a mut Context < 'b >) -> & 'a mut hyper_context < 'b > { unsafe { std :: mem :: transmute :: < & mut Context < '_ > , & mut hyper_context < '_ > > (cx) } } }
    };
}

impl_365!();
macro_rules! impl_75 {
    () => {
        impl < Fut : Future + fmt :: Debug > fmt :: Debug for Remote < Fut > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("Remote") . field (& self . future) . finish () } }
    };
}

impl_75!()
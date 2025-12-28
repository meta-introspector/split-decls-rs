macro_rules! impl_210 {
    () => {
        impl < Fut1 : Future , Fut2 : Future > Join < Fut1 , Fut2 > { pub (crate) fn new (fut1 : Fut1 , fut2 : Fut2) -> Self { Self { fut1 : maybe_done (fut1) , fut2 : maybe_done (fut2) } } }
    };
}

impl_210!()
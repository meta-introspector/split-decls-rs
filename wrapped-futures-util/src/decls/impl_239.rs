macro_rules! impl_239 {
    () => {
        impl < Fut1 : TryFuture , Fut2 : TryFuture > TryJoin < Fut1 , Fut2 > { pub (crate) fn new (fut1 : Fut1 , fut2 : Fut2) -> Self { Self { fut1 : try_maybe_done (fut1) , fut2 : try_maybe_done (fut2) } } }
    };
}

impl_239!();
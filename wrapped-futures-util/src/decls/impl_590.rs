macro_rules! impl_590 {
    () => {
        impl < St , Fut , F > FusedStream for AndThen < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (St :: Ok) -> Fut , Fut : TryFuture < Error = St :: Error > , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
    };
}

impl_590!()
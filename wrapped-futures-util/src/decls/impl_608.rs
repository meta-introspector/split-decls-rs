macro_rules! impl_608 {
    () => {
        impl < St , Fut , F > FusedStream for OrElse < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (St :: Error) -> Fut , Fut : TryFuture < Ok = St :: Ok > , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
    };
}

impl_608!();
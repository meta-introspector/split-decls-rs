macro_rules! impl_633 {
    () => {
        impl < St , Fut , F , T > FusedStream for TryFilterMap < St , Fut , F > where St : TryStream + FusedStream , Fut : TryFuture < Ok = Option < T > , Error = St :: Error > , F : FnMut (St :: Ok) -> Fut , { fn is_terminated (& self) -> bool { self . pending . is_none () && self . stream . is_terminated () } }
    };
}

impl_633!();
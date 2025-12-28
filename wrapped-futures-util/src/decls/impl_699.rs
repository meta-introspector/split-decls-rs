macro_rules! impl_699 {
    () => {
        impl < St , Fut , F > FusedStream for TrySkipWhile < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { fn is_terminated (& self) -> bool { self . pending_item . is_none () && self . stream . is_terminated () } }
    };
}

impl_699!();
macro_rules! impl_620 {
    () => {
        impl < St , Fut , F > FusedStream for TryFilter < St , Fut , F > where St : TryStream + FusedStream , F : FnMut (& St :: Ok) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . pending_fut . is_none () && self . stream . is_terminated () } }
    };
}

impl_620!()
macro_rules! impl_433 {
    () => {
        impl < St , Fut , F > FusedStream for SkipWhile < St , Fut , F > where St : FusedStream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . pending_item . is_none () && self . stream . is_terminated () } }
    };
}

impl_433!();
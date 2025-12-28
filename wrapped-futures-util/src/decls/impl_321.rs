macro_rules! impl_321 {
    () => {
        impl < St , Fut , F > FusedStream for Filter < St , Fut , F > where St : Stream + FusedStream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . pending_fut . is_none () && self . stream . is_terminated () } }
    };
}

impl_321!()
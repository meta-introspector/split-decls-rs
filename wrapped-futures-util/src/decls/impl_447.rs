macro_rules! impl_447 {
    () => {
        impl < St , Fut , F > FusedStream for TakeWhile < St , Fut , F > where St : FusedStream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done_taking || self . pending_item . is_none () && self . stream . is_terminated () } }
    };
}

impl_447!()
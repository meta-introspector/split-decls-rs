macro_rules! impl_366 {
    () => {
        impl < St , Fut , F > FusedFuture for ForEach < St , Fut , F > where St : FusedStream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
    };
}

impl_366!();
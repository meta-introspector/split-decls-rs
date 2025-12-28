macro_rules! impl_288 {
    () => {
        impl < St , C > FusedFuture for Collect < St , C > where St : FusedStream , C : Default + Extend < St :: Item > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_288!()
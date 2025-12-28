macro_rules! impl_661 {
    () => {
        impl < St , C > FusedFuture for TryCollect < St , C > where St : TryStream + FusedStream , C : Default + Extend < St :: Ok > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_661!()
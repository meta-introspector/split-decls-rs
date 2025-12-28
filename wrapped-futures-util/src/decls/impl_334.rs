macro_rules! impl_334 {
    () => {
        impl < St > FusedStream for Flatten < St , St :: Item > where St : FusedStream , St :: Item : Stream , { fn is_terminated (& self) -> bool { self . next . is_none () && self . stream . is_terminated () } }
    };
}

impl_334!();
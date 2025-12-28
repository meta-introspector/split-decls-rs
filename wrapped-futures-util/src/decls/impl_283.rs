macro_rules! impl_283 {
    () => {
        impl < St1 , St2 > FusedStream for Chain < St1 , St2 > where St1 : Stream , St2 : FusedStream < Item = St1 :: Item > , { fn is_terminated (& self) -> bool { self . first . is_none () && self . second . is_terminated () } }
    };
}

impl_283!();
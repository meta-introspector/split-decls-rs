macro_rules! deps {
    () => {
        FnMut1!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        impl < St , F > FusedStream for Map < St , F > where St : FusedStream , F : FnMut1 < St :: Item > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_384!()
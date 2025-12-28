macro_rules! deps {
    () => {
        SelectAll!();
    };
}

macro_rules! impl_893 {
    () => {
        deps!();
        impl < St : Stream + Unpin > FusedStream for SelectAll < St > { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_893!()
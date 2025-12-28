macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_793 {
    () => {
        deps!();
        impl < St1 , St2 > FusedStream for Select < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { fn is_terminated (& self) -> bool { self . inner . is_terminated () } }
    };
}

impl_793!()
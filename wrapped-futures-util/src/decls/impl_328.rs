macro_rules! deps {
    () => {
        FnMut1!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < St , Fut , F , T > FusedStream for FilterMap < St , Fut , F > where St : Stream + FusedStream , F : FnMut1 < St :: Item , Output = Fut > , Fut : Future < Output = Option < T > > , { fn is_terminated (& self) -> bool { self . pending . is_none () && self . stream . is_terminated () } }
    };
}

impl_328!()
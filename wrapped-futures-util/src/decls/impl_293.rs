macro_rules! impl_293 {
    () => {
        impl < St , A , B , FromA , FromB > FusedFuture for Unzip < St , FromA , FromB > where St : FusedStream < Item = (A , B) > , FromA : Default + Extend < A > , FromB : Default + Extend < B > , { fn is_terminated (& self) -> bool { self . stream . is_terminated () } }
    };
}

impl_293!();
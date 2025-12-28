macro_rules! impl_176 {
    () => {
        impl < F : FusedFuture > FusedFuture for OptionFuture < F > { fn is_terminated (& self) -> bool { match & self . inner { Some (x) => x . is_terminated () , None => true , } } }
    };
}

impl_176!()
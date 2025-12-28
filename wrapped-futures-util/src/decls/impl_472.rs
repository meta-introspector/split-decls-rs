macro_rules! impl_472 {
    () => {
        impl < St , Fut , T , F > FusedFuture for TryFold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : TryFuture < Ok = T > , { fn is_terminated (& self) -> bool { self . accum . is_none () && self . future . is_none () } }
    };
}

impl_472!();
macro_rules! impl_729 {
    () => {
        impl < St , Fut , F > FusedFuture for TryAll < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
    };
}

impl_729!();
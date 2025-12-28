macro_rules! impl_348 {
    () => {
        impl < St , Fut , F > FusedFuture for Any < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
    };
}

impl_348!()
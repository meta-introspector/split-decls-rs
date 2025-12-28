macro_rules! impl_354 {
    () => {
        impl < St , Fut , F > FusedFuture for All < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { fn is_terminated (& self) -> bool { self . done && self . future . is_none () } }
    };
}

impl_354!()
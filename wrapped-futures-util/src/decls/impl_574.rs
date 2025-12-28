macro_rules! impl_574 {
    () => {
        impl < St , Fut , F , E > FusedFuture for TryForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = Result < () , E > > , { fn is_terminated (& self) -> bool { self . stream . is_none () && self . futures . is_empty () } }
    };
}

impl_574!()
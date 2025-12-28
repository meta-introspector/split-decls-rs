macro_rules! impl_342 {
    () => {
        impl < St , Fut , T , F > FusedFuture for Fold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : Future < Output = T > , { fn is_terminated (& self) -> bool { self . accum . is_none () && self . future . is_none () } }
    };
}

impl_342!()
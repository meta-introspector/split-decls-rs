macro_rules! impl_550 {
    () => {
        impl < St , Fut , F > FusedFuture for ForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { fn is_terminated (& self) -> bool { self . stream . is_none () && self . futures . is_empty () } }
    };
}

impl_550!();
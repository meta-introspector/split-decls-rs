macro_rules! impl_460 {
    () => {
        impl < St , Fut , F > FusedStream for Then < St , Fut , F > where St : FusedStream , F : FnMut (St :: Item) -> Fut , Fut : Future , { fn is_terminated (& self) -> bool { self . future . is_none () && self . stream . is_terminated () } }
    };
}

impl_460!();
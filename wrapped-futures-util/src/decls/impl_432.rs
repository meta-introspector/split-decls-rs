macro_rules! impl_432 {
    () => {
        impl < St , Fut , F > SkipWhile < St , Fut , F > where St : Stream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_skipping : false } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_432!();
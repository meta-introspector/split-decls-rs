macro_rules! impl_445 {
    () => {
        impl < St , Fut , F > TakeWhile < St , Fut , F > where St : Stream , F : FnMut (& St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_taking : false } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_445!();
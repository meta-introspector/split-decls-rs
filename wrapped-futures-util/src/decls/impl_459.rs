macro_rules! impl_459 {
    () => {
        impl < St , Fut , F > Then < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , future : None , f } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_459!();
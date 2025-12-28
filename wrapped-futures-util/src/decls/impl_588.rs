macro_rules! impl_588 {
    () => {
        impl < St , Fut , F > AndThen < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : TryFuture < Error = St :: Error > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , future : None , f } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_588!();
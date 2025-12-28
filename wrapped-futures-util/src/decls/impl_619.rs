macro_rules! impl_619 {
    () => {
        impl < St , Fut , F > TryFilter < St , Fut , F > where St : TryStream , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_619!();
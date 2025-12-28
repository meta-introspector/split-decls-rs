macro_rules! impl_632 {
    () => {
        impl < St , Fut , F > TryFilterMap < St , Fut , F > { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending : None } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_632!()
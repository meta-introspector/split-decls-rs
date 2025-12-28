macro_rules! impl_383 {
    () => {
        impl < St , F > Map < St , F > { pub (crate) fn new (stream : St , f : F) -> Self { Self { stream , f } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_383!()
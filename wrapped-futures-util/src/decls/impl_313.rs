macro_rules! impl_313 {
    () => {
        impl < St : Stream > Enumerate < St > { pub (super) fn new (stream : St) -> Self { Self { stream , count : 0 } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_313!()
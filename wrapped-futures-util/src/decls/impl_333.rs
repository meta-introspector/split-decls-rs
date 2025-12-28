macro_rules! impl_333 {
    () => {
        impl < St , U > Flatten < St , U > { pub (super) fn new (stream : St) -> Self { Self { stream , next : None } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_333!();
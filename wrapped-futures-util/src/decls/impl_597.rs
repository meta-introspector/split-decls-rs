macro_rules! impl_597 {
    () => {
        impl < St > IntoStream < St > { # [inline] pub (super) fn new (stream : St) -> Self { Self { stream } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_597!();
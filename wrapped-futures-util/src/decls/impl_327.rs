macro_rules! impl_327 {
    () => {
        impl < St , Fut , F > FilterMap < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending : None } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_327!();
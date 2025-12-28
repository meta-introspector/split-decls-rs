macro_rules! impl_625 {
    () => {
        impl < St , Si , Item > TryForward < St , Si , Item > { pub (crate) fn new (stream : St , sink : Si) -> Self { Self { sink : Some (sink) , stream : Fuse :: new (IntoStream :: new (stream)) , buffered_item : None } } }
    };
}

impl_625!()
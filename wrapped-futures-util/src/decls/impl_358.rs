macro_rules! impl_358 {
    () => {
        impl < St , Si , Item > Forward < St , Si , Item > { pub (crate) fn new (stream : St , sink : Si) -> Self { Self { sink : Some (sink) , stream : Fuse :: new (stream) , buffered_item : None } } }
    };
}

impl_358!()
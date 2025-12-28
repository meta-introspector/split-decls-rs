macro_rules! impl_627 {
    () => {
        impl < T > ReaderStream < T > { pub (crate) fn new (reader : T) -> Self { Self { buf : [0 ; 2048] , reader , } } }
    };
}

impl_627!()
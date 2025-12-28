macro_rules! impl_303 {
    () => {
        impl < St : Stream > Count < St > { pub (super) fn new (stream : St) -> Self { Self { stream , count : 0 } } }
    };
}

impl_303!();
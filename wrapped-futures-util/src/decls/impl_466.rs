macro_rules! impl_466 {
    () => {
        impl < St , Fut , F > TryForEach < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : TryFuture < Ok = () > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , future : None } } }
    };
}

impl_466!();
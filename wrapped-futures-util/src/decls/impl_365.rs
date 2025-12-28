macro_rules! impl_365 {
    () => {
        impl < St , Fut , F > ForEach < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = () > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , future : None } } }
    };
}

impl_365!();
macro_rules! impl_347 {
    () => {
        impl < St , Fut , F > Any < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
    };
}

impl_347!()
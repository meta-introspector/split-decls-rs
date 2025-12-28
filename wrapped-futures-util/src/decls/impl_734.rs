macro_rules! impl_734 {
    () => {
        impl < St , Fut , F > TryAny < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
    };
}

impl_734!()
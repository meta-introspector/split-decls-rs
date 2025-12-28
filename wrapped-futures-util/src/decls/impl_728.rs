macro_rules! impl_728 {
    () => {
        impl < St , Fut , F > TryAll < St , Fut , F > where St : TryStream , F : FnMut (St :: Ok) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
    };
}

impl_728!()
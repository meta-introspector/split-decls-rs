macro_rules! impl_353 {
    () => {
        impl < St , Fut , F > All < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , done : false , future : None } } }
    };
}

impl_353!()
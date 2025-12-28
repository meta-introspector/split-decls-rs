macro_rules! impl_341 {
    () => {
        impl < St , Fut , T , F > Fold < St , Fut , T , F > where St : Stream , F : FnMut (T , St :: Item) -> Fut , Fut : Future < Output = T > , { pub (super) fn new (stream : St , f : F , t : T) -> Self { Self { stream , f , accum : Some (t) , future : None } } }
    };
}

impl_341!()
macro_rules! impl_495 {
    () => {
        impl < B , St , S , Fut , F > Scan < St , S , Fut , F > where St : Stream , F : FnMut (S , St :: Item) -> Fut , Fut : Future < Output = Option < (S , B) > > , { pub (super) fn new (stream : St , initial_state : S , f : F) -> Self { Self { stream , f , state : UnfoldState :: Value { value : initial_state } } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_495!()
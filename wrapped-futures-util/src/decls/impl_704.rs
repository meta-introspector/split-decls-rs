macro_rules! impl_704 {
    () => {
        impl < St , Fut , F > TryTakeWhile < St , Fut , F > where St : TryStream , F : FnMut (& St :: Ok) -> Fut , Fut : TryFuture < Ok = bool , Error = St :: Error > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None , done_taking : false } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_704!()
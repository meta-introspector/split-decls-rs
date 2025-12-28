macro_rules! deps {
    () => {
        FnMut1!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl < St , Fut , F > Filter < St , Fut , F > where St : Stream , F : for < 'a > FnMut1 < & 'a St :: Item , Output = Fut > , Fut : Future < Output = bool > , { pub (super) fn new (stream : St , f : F) -> Self { Self { stream , f , pending_fut : None , pending_item : None } } delegate_access_inner ! (stream , St , ()) ; }
    };
}

impl_320!();
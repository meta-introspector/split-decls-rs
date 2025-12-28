macro_rules! deps {
    () => {
        Ready!();
        Sink!();
    };
}

macro_rules! impl_972 {
    () => {
        deps!();
        impl < Si , Item , U , Fut , F , E > With < Si , Item , U , Fut , F > where Si : Sink < Item > , F : FnMut (U) -> Fut , Fut : Future < Output = Result < Item , E > > , E : From < Si :: Error > , { delegate_access_inner ! (sink , Si , ()) ; # [doc = " Completes the processing of previous item if any."] fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , E > > { let mut this = self . project () ; let item = match this . state . as_mut () . as_pin_mut () { None => return Poll :: Ready (Ok (())) , Some (fut) => ready ! (fut . poll (cx)) ? , } ; this . state . set (None) ; this . sink . start_send (item) ? ; Poll :: Ready (Ok (())) } }
    };
}

impl_972!()
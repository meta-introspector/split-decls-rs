macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_948 {
    () => {
        deps!();
        impl < Si , F , E , Item > Sink < Item > for SinkMapErr < Si , F > where Si : Sink < Item > , F : FnOnce (Si :: Error) -> E , { type Error = E ; fn poll_ready (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . as_mut () . project () . sink . poll_ready (cx) . map_err (| e | self . as_mut () . take_f () (e)) } fn start_send (mut self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { self . as_mut () . project () . sink . start_send (item) . map_err (| e | self . as_mut () . take_f () (e)) } fn poll_flush (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . as_mut () . project () . sink . poll_flush (cx) . map_err (| e | self . as_mut () . take_f () (e)) } fn poll_close (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . as_mut () . project () . sink . poll_close (cx) . map_err (| e | self . as_mut () . take_f () (e)) } }
    };
}

impl_948!()
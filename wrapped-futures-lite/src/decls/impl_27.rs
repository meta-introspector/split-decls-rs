macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < Fut : Future > Future for Fuse < Fut > { type Output = Fut :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Fut :: Output > { match self . as_mut () . project () . inner . as_pin_mut () . map (| f | f . poll (cx)) { Some (Poll :: Ready (output)) => { self . project () . inner . set (None) ; Poll :: Ready (output) } Some (Poll :: Pending) | None => Poll :: Pending , } } }
    };
}

impl_27!()
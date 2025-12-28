macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < Fut : Future > Future for Fuse < Fut > { type Output = Fut :: Output ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Fut :: Output > { match self . as_mut () . project () . inner . as_pin_mut () { Some (fut) => fut . poll (cx) . map (| output | { self . project () . inner . set (None) ; output }) , None => Poll :: Pending , } } }
    };
}

impl_50!();
macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < F : Future > Future for OptionFuture < F > { type Output = Option < F :: Output > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . inner . as_pin_mut () { Some (x) => x . poll (cx) . map (Some) , None => Poll :: Ready (None) , } } }
    };
}

impl_175!()
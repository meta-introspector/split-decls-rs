macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < T , F > Future for PollOnce < F > where F : Future < Output = T > , { type Output = Option < T > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . project () . f . poll (cx) { Poll :: Ready (t) => Poll :: Ready (Some (t)) , Poll :: Pending => Poll :: Ready (None) , } } }
    };
}

impl_6!();
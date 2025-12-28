macro_rules! deps {
    () => {
        Race!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl < Fut > Future for Race < Fut > where Fut : Future , { type Output = Fut :: Output ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; assert ! (!* this . done , "Futures must not be polled after completing") ; for index in this . indexer . iter () { let fut = utils :: get_pin_mut_from_vec (this . futures . as_mut () , index) . unwrap () ; match fut . poll (cx) { Poll :: Ready (item) => { * this . done = true ; return Poll :: Ready (item) ; } Poll :: Pending => continue , } } Poll :: Pending } }
    };
}

impl_280!();
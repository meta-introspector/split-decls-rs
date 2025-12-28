macro_rules! deps {
    () => {
        Chain!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < S : Stream , const N : usize > Stream for Chain < S , N > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; assert ! (!* this . done , "Stream should not be polled after completion") ; loop { if this . index == this . len { * this . done = true ; return Poll :: Ready (None) ; } let stream = utils :: iter_pin_mut (this . streams . as_mut ()) . nth (* this . index) . unwrap () ; match stream . poll_next (cx) { Poll :: Ready (Some (item)) => return Poll :: Ready (Some (item)) , Poll :: Ready (None) => { * this . index += 1 ; continue ; } Poll :: Pending => return Poll :: Pending , } } } }
    };
}

impl_403!();
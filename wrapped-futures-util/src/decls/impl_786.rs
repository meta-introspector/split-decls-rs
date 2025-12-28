macro_rules! deps {
    () => {
        Ready!();
        Pending!();
    };
}

macro_rules! impl_786 {
    () => {
        deps!();
        impl < T , S > Stream for PollImmediate < S > where S : Stream < Item = T > , { type Item = Poll < T > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let stream = match this . stream . as_mut () . as_pin_mut () { None => return Poll :: Ready (None) , Some (inner) => inner , } ; match stream . poll_next (cx) { Poll :: Ready (Some (t)) => Poll :: Ready (Some (Poll :: Ready (t))) , Poll :: Ready (None) => { this . stream . set (None) ; Poll :: Ready (None) } Poll :: Pending => Poll :: Ready (Some (Poll :: Pending)) , } } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . as_ref () . map_or ((0 , Some (0)) , Stream :: size_hint) } }
    };
}

impl_786!();
macro_rules! deps {
    () => {
        FromIter!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < I : Iterator > Stream for FromIter < I > { type Item = I :: Item ; fn poll_next (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (self . iter . next ()) } }
    };
}

impl_56!();
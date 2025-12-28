macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < I : Iterator > Stream for Iter < I > { type Item = I :: Item ; fn poll_next (mut self : Pin < & mut Self > , _cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (self . iter . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_50!();
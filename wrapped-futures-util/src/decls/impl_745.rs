macro_rules! deps {
    () => {
        Ready!();
        Iter!();
    };
}

macro_rules! impl_745 {
    () => {
        deps!();
        impl < I > Stream for Iter < I > where I : Iterator , { type Item = I :: Item ; fn poll_next (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < I :: Item > > { Poll :: Ready (self . iter . next ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . iter . size_hint () } }
    };
}

impl_745!();
macro_rules! deps {
    () => {
        Ready!();
        Empty!();
    };
}

macro_rules! impl_763 {
    () => {
        deps!();
        impl < T > Stream for Empty < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (None) } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_763!();
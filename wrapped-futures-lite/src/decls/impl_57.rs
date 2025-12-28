macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T > Stream for Pending < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < T > > { Poll :: Pending } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }
    };
}

impl_57!()
macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < S : ? Sized + Stream + Unpin > Stream for & mut S { type Item = S :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { S :: poll_next (Pin :: new (& mut * * self) , cx) } fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } }
    };
}

impl_15!();
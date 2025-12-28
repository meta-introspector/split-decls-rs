macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < P > Stream for Pin < P > where P : DerefMut + Unpin , P :: Target : Stream , { type Item = < P :: Target as Stream > :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . get_mut () . as_mut () . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { (* * self) . size_hint () } }
    };
}

impl_16!()
macro_rules! deps {
    () => {
        InterleavePending!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < St : Stream > Stream for InterleavePending < St > { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . poll_with (cx , St :: poll_next) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
    };
}

impl_88!()
macro_rules! deps {
    () => {
        Buf!();
        IntoIter!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T : Buf > Iterator for IntoIter < T > { type Item = u8 ; fn next (& mut self) -> Option < u8 > { if ! self . inner . has_remaining () { return None ; } let b = self . inner . chunk () [0] ; self . inner . advance (1) ; Some (b) } fn size_hint (& self) -> (usize , Option < usize >) { let rem = self . inner . remaining () ; (rem , Some (rem)) } }
    };
}

impl_30!()
macro_rules! deps {
    () => {
        Repeat!();
        Ready!();
    };
}

macro_rules! impl_750 {
    () => {
        deps!();
        impl < T > Stream for Repeat < T > where T : Clone , { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (Some (self . item . clone ())) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
    };
}

impl_750!()
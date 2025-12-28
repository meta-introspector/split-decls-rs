macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T : Clone > Stream for Repeat < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (Some (self . item . clone ())) } fn size_hint (& self) -> (usize , Option < usize >) { (usize :: MAX , None) } }
    };
}

impl_66!()
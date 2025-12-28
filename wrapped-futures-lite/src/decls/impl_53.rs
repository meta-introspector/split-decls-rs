macro_rules! impl_53 {
    () => {
        impl < T > Stream for Once < T > { type Item = T ; fn poll_next (self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < T > > { Poll :: Ready (self . project () . value . take ()) } fn size_hint (& self) -> (usize , Option < usize >) { if self . value . is_some () { (1 , Some (1)) } else { (0 , Some (0)) } } }
    };
}

impl_53!()
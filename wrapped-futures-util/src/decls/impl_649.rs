macro_rules! deps {
    () => {
        Ready!();
        Single!();
    };
}

macro_rules! impl_649 {
    () => {
        deps!();
        impl < T > Stream for Single < T > { type Item = T ; fn poll_next (mut self : Pin < & mut Self > , _ : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { Poll :: Ready (self . 0 . take ()) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . as_ref () . map_or ((0 , Some (0)) , | _ | (1 , Some (1))) } }
    };
}

impl_649!()
macro_rules! impl_599 {
    () => {
        impl < St : TryStream > Stream for IntoStream < St > { type Item = Result < St :: Ok , St :: Error > ; # [inline] fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { self . project () . stream . try_poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
    };
}

impl_599!();
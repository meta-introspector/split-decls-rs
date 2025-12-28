macro_rules! impl_114 {
    () => {
        impl < S , F , T > Stream for Map < S , F > where S : Stream , F : FnMut (S :: Item) -> T , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let next = ready ! (this . stream . poll_next (cx)) ; Poll :: Ready (next . map (this . f)) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
    };
}

impl_114!()
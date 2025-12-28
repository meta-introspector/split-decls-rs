macro_rules! deps {
    () => {
        Ready!();
        FnMut1!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        impl < St , F > Stream for Map < St , F > where St : Stream , F : FnMut1 < St :: Item > , { type Item = F :: Output ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; let res = ready ! (this . stream . as_mut () . poll_next (cx)) ; Poll :: Ready (res . map (| x | this . f . call_mut (x))) } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
    };
}

impl_385!();
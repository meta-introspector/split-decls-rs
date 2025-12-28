macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl < St : Stream > Stream for Skip < St > { type Item = St :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St :: Item > > { let mut this = self . project () ; while * this . remaining > 0 { if ready ! (this . stream . as_mut () . poll_next (cx)) . is_some () { * this . remaining -= 1 ; } else { return Poll :: Ready (None) ; } } this . stream . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_sub (self . remaining) ; let upper = upper . map (| x | x . saturating_sub (self . remaining)) ; (lower , upper) } }
    };
}

impl_427!();
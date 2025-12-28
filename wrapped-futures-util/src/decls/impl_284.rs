macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl < St1 , St2 > Stream for Chain < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { type Item = St1 :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if let Some (first) = this . first . as_mut () . as_pin_mut () { if let Some (item) = ready ! (first . poll_next (cx)) { return Poll :: Ready (Some (item)) ; } this . first . set (None) ; } this . second . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { if let Some (first) = & self . first { let (first_lower , first_upper) = first . size_hint () ; let (second_lower , second_upper) = self . second . size_hint () ; let lower = first_lower . saturating_add (second_lower) ; let upper = match (first_upper , second_upper) { (Some (x) , Some (y)) => x . checked_add (y) , _ => None , } ; (lower , upper) } else { self . second . size_hint () } } }
    };
}

impl_284!();
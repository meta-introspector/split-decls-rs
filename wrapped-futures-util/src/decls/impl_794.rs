macro_rules! deps {
    () => {
        Select!();
    };
}

macro_rules! impl_794 {
    () => {
        deps!();
        impl < St1 , St2 > Stream for Select < St1 , St2 > where St1 : Stream , St2 : Stream < Item = St1 :: Item > , { type Item = St1 :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < St1 :: Item > > { let this = self . project () ; this . inner . poll_next (cx) } }
    };
}

impl_794!();
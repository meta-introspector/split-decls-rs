macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl < St > Future for TryConcat < St > where St : TryStream , St :: Ok : Extend < < St :: Ok as IntoIterator > :: Item > + IntoIterator + Default , { type Output = Result < St :: Ok , St :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (Ok (loop { if let Some (x) = ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { if let Some (a) = this . accum { a . extend (x) } else { * this . accum = Some (x) } } else { break this . accum . take () . unwrap_or_default () ; } })) } }
    };
}

impl_666!()
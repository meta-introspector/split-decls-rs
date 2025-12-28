macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_298 {
    () => {
        deps!();
        impl < St > Future for Concat < St > where St : Stream , St :: Item : Extend < < St :: Item as IntoIterator > :: Item > + IntoIterator + Default , { type Output = St :: Item ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { None => return Poll :: Ready (this . accum . take () . unwrap_or_default ()) , Some (e) => { if let Some (a) = this . accum { a . extend (e) } else { * this . accum = Some (e) } } } } } }
    };
}

impl_298!();
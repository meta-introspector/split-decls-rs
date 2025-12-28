macro_rules! deps {
    () => {
        Next!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < St : ? Sized + Stream + Unpin > Future for Next < '_ , St > { type Output = Option < St :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . stream . poll_next_unpin (cx) } }
    };
}

impl_393!()
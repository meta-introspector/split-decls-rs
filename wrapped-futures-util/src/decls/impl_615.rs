macro_rules! deps {
    () => {
        TryNext!();
    };
}

macro_rules! impl_615 {
    () => {
        deps!();
        impl < St : ? Sized + TryStream + Unpin > Future for TryNext < '_ , St > { type Output = Result < Option < St :: Ok > , St :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { self . stream . try_poll_next_unpin (cx) ? . map (Ok) } }
    };
}

impl_615!();
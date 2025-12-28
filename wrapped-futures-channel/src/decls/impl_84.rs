macro_rules! deps {
    () => {
        Recv!();
        RecvError!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < St : ? Sized + Stream + Unpin > Future for Recv < '_ , St > { type Output = Result < St :: Item , RecvError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match Pin :: new (& mut self . stream) . poll_next (cx) { Poll :: Ready (Some (msg)) => Poll :: Ready (Ok (msg)) , Poll :: Ready (None) => Poll :: Ready (Err (RecvError)) , Poll :: Pending => Poll :: Pending , } } }
    };
}

impl_84!()
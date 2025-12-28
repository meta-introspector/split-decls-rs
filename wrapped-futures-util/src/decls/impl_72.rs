macro_rules! deps {
    () => {
        RemoteHandle!();
        Ready!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T : 'static > Future for RemoteHandle < T > { type Output = T ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < T > { match ready ! (self . rx . poll_unpin (cx)) { Ok (Ok (output)) => Poll :: Ready (output) , Ok (Err (e)) => panic :: resume_unwind (e) , Err (e) => panic :: resume_unwind (Box :: new (e)) , } } }
    };
}

impl_72!();
macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_305 {
    () => {
        deps!();
        impl < St : Stream > Future for Count < St > { type Output = usize ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (_) => * this . count += 1 , None => break * this . count , } }) } }
    };
}

impl_305!();
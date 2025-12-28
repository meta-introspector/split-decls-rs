macro_rules! impl_97 {
    () => {
        impl < S : Stream + ? Sized > Future for CountFuture < S > { type Output = usize ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . as_mut () . project () . stream . poll_next (cx)) { None => return Poll :: Ready (self . count) , Some (_) => * self . as_mut () . project () . count += 1 , } } } }
    };
}

impl_97!();
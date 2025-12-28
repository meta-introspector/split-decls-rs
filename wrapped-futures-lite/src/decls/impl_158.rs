macro_rules! deps {
    () => {
        NthFuture!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < S > Future for NthFuture < '_ , S > where S : Stream + Unpin + ? Sized , { type Output = Option < S :: Item > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { match ready ! (self . stream . poll_next (cx)) { Some (v) => match self . n { 0 => return Poll :: Ready (Some (v)) , _ => self . n -= 1 , } , None => return Poll :: Ready (None) , } } } }
    };
}

impl_158!()
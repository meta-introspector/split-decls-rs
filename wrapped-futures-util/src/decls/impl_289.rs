macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_289 {
    () => {
        deps!();
        impl < St , C > Future for Collect < St , C > where St : Stream , C : Default + Extend < St :: Item > , { type Output = C ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < C > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (e) => this . collection . extend (Some (e)) , None => return Poll :: Ready (self . finish ()) , } } } }
    };
}

impl_289!();
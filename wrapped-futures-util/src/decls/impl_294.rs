macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < St , A , B , FromA , FromB > Future for Unzip < St , FromA , FromB > where St : Stream < Item = (A , B) > , FromA : Default + Extend < A > , FromB : Default + Extend < B > , { type Output = (FromA , FromB) ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < (FromA , FromB) > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (e) => { this . left . extend (Some (e . 0)) ; this . right . extend (Some (e . 1)) ; } None => return Poll :: Ready (self . finish ()) , } } } }
    };
}

impl_294!()
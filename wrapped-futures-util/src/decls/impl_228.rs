macro_rules! deps {
    () => {
        Either!();
        Select!();
        Ready!();
        Pending!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl < A , B > Future for Select < A , B > where A : Future + Unpin , B : Future + Unpin , { type Output = Either < (A :: Output , B) , (B :: Output , A) > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { # [doc = " When compiled with `-C opt-level=z`, this function will help the compiler eliminate the `None` branch, where"] # [doc = " `Option::unwrap` does not."] # [inline (always)] fn unwrap_option < T > (value : Option < T >) -> T { match value { None => unreachable ! () , Some (value) => value , } } let (a , b) = self . inner . as_mut () . expect ("cannot poll Select twice") ; if let Poll :: Ready (val) = a . poll_unpin (cx) { return Poll :: Ready (Either :: Left ((val , unwrap_option (self . inner . take ()) . 1))) ; } if let Poll :: Ready (val) = b . poll_unpin (cx) { return Poll :: Ready (Either :: Right ((val , unwrap_option (self . inner . take ()) . 0))) ; } Poll :: Pending } }
    };
}

impl_228!();
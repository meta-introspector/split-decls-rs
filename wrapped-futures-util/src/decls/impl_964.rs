macro_rules! deps {
    () => {
        Empty!();
        Sink!();
        Ready!();
    };
}

macro_rules! impl_964 {
    () => {
        deps!();
        impl < T , F , Fut , Item , E > Sink < Item > for Unfold < T , F , Fut > where F : FnMut (T , Item) -> Fut , Fut : Future < Output = Result < T , E > > , { type Error = E ; fn poll_ready (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_flush (cx) } fn start_send (self : Pin < & mut Self > , item : Item) -> Result < () , Self :: Error > { let mut this = self . project () ; let future = match this . state . as_mut () . take_value () { Some (value) => (this . function) (value , item) , None => panic ! ("start_send called without poll_ready being called first") , } ; this . state . set (UnfoldState :: Future { future }) ; Ok (()) } fn poll_flush (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let mut this = self . project () ; Poll :: Ready (if let Some (future) = this . state . as_mut () . project_future () { match ready ! (future . poll (cx)) { Ok (state) => { this . state . set (UnfoldState :: Value { value : state }) ; Ok (()) } Err (err) => { this . state . set (UnfoldState :: Empty) ; Err (err) } } } else { Ok (()) }) } fn poll_close (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { self . poll_flush (cx) } }
    };
}

impl_964!();
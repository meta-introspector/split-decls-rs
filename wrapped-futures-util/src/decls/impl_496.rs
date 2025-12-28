macro_rules! deps {
    () => {
        Ready!();
        Empty!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < B , St , S , Fut , F > Stream for Scan < St , S , Fut , F > where St : Stream , F : FnMut (S , St :: Item) -> Fut , Fut : Future < Output = Option < (S , B) > > , { type Item = B ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < B > > { if self . is_done_taking () { return Poll :: Ready (None) ; } let mut this = self . project () ; Poll :: Ready (loop { if let Some (fut) = this . state . as_mut () . project_future () { match ready ! (fut . poll (cx)) { None => { this . state . set (UnfoldState :: Empty) ; break None ; } Some ((state , item)) => { this . state . set (UnfoldState :: Value { value : state }) ; break Some (item) ; } } } else if let Some (item) = ready ! (this . stream . as_mut () . poll_next (cx)) { let state = this . state . as_mut () . take_value () . unwrap () ; this . state . set (UnfoldState :: Future { future : (this . f) (state , item) }) } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { if self . is_done_taking () { (0 , Some (0)) } else { self . stream . size_hint () } } }
    };
}

impl_496!()
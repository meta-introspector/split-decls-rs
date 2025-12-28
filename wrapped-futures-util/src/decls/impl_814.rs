macro_rules! deps {
    () => {
        Ready!();
        Empty!();
    };
}

macro_rules! impl_814 {
    () => {
        deps!();
        impl < T , F , Fut , Item > Stream for Unfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Option < (Item , T) > > , { type Item = Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if let Some (state) = this . state . as_mut () . take_value () { this . state . set (UnfoldState :: Future { future : (this . f) (state) }) ; } let step = match this . state . as_mut () . project_future () { Some (fut) => ready ! (fut . poll (cx)) , None => panic ! ("Unfold must not be polled after it returned `Poll::Ready(None)`") , } ; if let Some ((item , next_state)) = step { this . state . set (UnfoldState :: Value { value : next_state }) ; Poll :: Ready (Some (item)) } else { this . state . set (UnfoldState :: Empty) ; Poll :: Ready (None) } } }
    };
}

impl_814!()
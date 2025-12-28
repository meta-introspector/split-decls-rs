macro_rules! impl_74 {
    () => {
        impl < T , F , Fut , Item > Stream for Unfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Option < (Item , T) > > , { type Item = Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if let Some (state) = this . state . take () { this . fut . set (Some ((this . f) (state))) ; } let step = ready ! (this . fut . as_mut () . as_pin_mut () . expect ("`Unfold` must not be polled after it returned `Poll::Ready(None)`") . poll (cx)) ; this . fut . set (None) ; if let Some ((item , next_state)) = step { * this . state = Some (next_state) ; Poll :: Ready (Some (item)) } else { Poll :: Ready (None) } } }
    };
}

impl_74!()
macro_rules! impl_78 {
    () => {
        impl < T , E , F , Fut , Item > Stream for TryUnfold < T , F , Fut > where F : FnMut (T) -> Fut , Fut : Future < Output = Result < Option < (Item , T) > , E > > , { type Item = Result < Item , E > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if let Some (state) = this . state . take () { this . fut . set (Some ((this . f) (state))) ; } match this . fut . as_mut () . as_pin_mut () { None => { Poll :: Ready (None) } Some (future) => { let step = ready ! (future . poll (cx)) ; this . fut . set (None) ; match step { Ok (Some ((item , next_state))) => { * this . state = Some (next_state) ; Poll :: Ready (Some (Ok (item))) } Ok (None) => Poll :: Ready (None) , Err (e) => Poll :: Ready (Some (Err (e))) , } } } } }
    };
}

impl_78!();
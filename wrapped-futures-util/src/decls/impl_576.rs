macro_rules! deps {
    () => {
        Pending!();
        Ready!();
    };
}

macro_rules! impl_576 {
    () => {
        deps!();
        impl < St , Fut , F , E > Future for TryForEachConcurrent < St , Fut , F > where St : Stream , F : FnMut (St :: Item) -> Fut , Fut : Future < Output = Result < () , E > > , { type Output = Result < () , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { let mut made_progress_this_iter = false ; if this . limit . map (| limit | limit . get () > this . futures . len ()) . unwrap_or (true) { let poll_res = match this . stream . as_mut () . as_pin_mut () { Some (stream) => stream . poll_next (cx) , None => Poll :: Ready (None) , } ; let elem = match poll_res { Poll :: Ready (Some (elem)) => { made_progress_this_iter = true ; Some (elem) } Poll :: Ready (None) => { this . stream . set (None) ; None } Poll :: Pending => None , } ; if let Some (elem) = elem { this . futures . push ((this . f) (elem)) ; } } match this . futures . poll_next_unpin (cx) { Poll :: Ready (Some (Ok (()))) => made_progress_this_iter = true , Poll :: Ready (None) => { if this . stream . is_none () { return Poll :: Ready (Ok (())) ; } } Poll :: Pending => { } Poll :: Ready (Some (Err (e))) => { this . stream . set (None) ; drop (core :: mem :: take (this . futures)) ; return Poll :: Ready (Err (e)) ; } } if ! made_progress_this_iter { return Poll :: Pending ; } } } }
    };
}

impl_576!();
macro_rules! deps {
    () => {
        Ready!();
        SelectOk!();
        Pending!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl < Fut : TryFuture + Unpin > Future for SelectOk < Fut > { type Output = Result < (Fut :: Ok , Vec < Fut >) , Fut :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { loop { let item = self . inner . iter_mut () . enumerate () . find_map (| (i , f) | match f . try_poll_unpin (cx) { Poll :: Pending => None , Poll :: Ready (e) => Some ((i , e)) , }) ; match item { Some ((idx , res)) => { drop (self . inner . remove (idx)) ; match res { Ok (e) => { let rest = mem :: take (& mut self . inner) ; return Poll :: Ready (Ok ((e , rest))) ; } Err (e) => { if self . inner . is_empty () { return Poll :: Ready (Err (e)) ; } } } } None => { return Poll :: Pending ; } } } } }
    };
}

impl_262!();
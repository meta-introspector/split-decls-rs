macro_rules! deps {
    () => {
        Ready!();
        SelectAll!();
        Pending!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < Fut : Future + Unpin > Future for SelectAll < Fut > { type Output = (Fut :: Output , usize , Vec < Fut >) ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let item = self . inner . iter_mut () . enumerate () . find_map (| (i , f) | match f . poll_unpin (cx) { Poll :: Pending => None , Poll :: Ready (e) => Some ((i , e)) , }) ; match item { Some ((idx , res)) => { # [allow (clippy :: let_underscore_future)] let _ = self . inner . swap_remove (idx) ; let rest = mem :: take (& mut self . inner) ; Poll :: Ready ((res , idx , rest)) } None => Poll :: Pending , } } }
    };
}

impl_235!()
// Generated macro for impl_1285 (impl)
macro_rules! Depcrate_stream_try_stream_try_filter_mapimpl_1285 {
() => {
// Module: crate::stream::try_stream::try_filter_map
// Provides: {"impl_1285"}
// Dependencies: {}
impl < St , Fut , F , T > Stream for TryFilterMap < St , Fut , F > where St : TryStream , Fut : TryFuture < Ok = Option < T > , Error = St :: Error > , F : FnMut (St :: Ok) -> Fut , { type Item = Result < T , St :: Error > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; Poll :: Ready (loop { if let Some (p) = this . pending . as_mut () . as_pin_mut () { let res = ready ! (p . try_poll (cx)) ; this . pending . set (None) ; let item = res ? ; if item . is_some () { break item . map (Ok) ; } } else if let Some (item) = ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { this . pending . set (Some ((this . f) (item))) ; } else { break None ; } }) } fn size_hint (& self) -> (usize , Option < usize >) { let pending_len = usize :: from (self . pending . is_some ()) ; let (_ , upper) = self . stream . size_hint () ; let upper = match upper { Some (x) => x . checked_add (pending_len) , None => None , } ; (0 , upper) } }
};
}

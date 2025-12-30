// Generated macro for impl_2065 (impl)
macro_rules! Depcrate_compat_compat01as03impl_2065 {
() => {
// Module: crate::compat::compat01as03
// Provides: {"impl_2065"}
// Dependencies: {}
impl < St : Stream01 > Stream03 for Compat01As03 < St > { type Item = Result < St :: Item , St :: Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> task03 :: Poll < Option < Self :: Item > > { match self . in_notify (cx , Stream01 :: poll) ? { Async01 :: Ready (Some (t)) => task03 :: Poll :: Ready (Some (Ok (t))) , Async01 :: Ready (None) => task03 :: Poll :: Ready (None) , Async01 :: NotReady => task03 :: Poll :: Pending , } } }
};
}

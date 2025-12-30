// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < T > WeakSender < T > { # [doc = " Upgrade the [`WeakSender`] into a [`Sender`]."] pub fn upgrade (& self) -> Option < Sender < T > > { if self . channel . queue . is_closed () { None } else { match self . channel . sender_count . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | count | if count == 0 { None } else { Some (count + 1) } ,) { Err (_) => None , Ok (new_value) if new_value > usize :: MAX / 2 => { abort () ; } Ok (_) => Some (Sender { channel : self . channel . clone () , }) , } } } }
};
}

// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < T > WeakReceiver < T > { # [doc = " Upgrade the [`WeakReceiver`] into a [`Receiver`]."] pub fn upgrade (& self) -> Option < Receiver < T > > { if self . channel . queue . is_closed () { None } else { match self . channel . receiver_count . fetch_update (Ordering :: Relaxed , Ordering :: Relaxed , | count | if count == 0 { None } else { Some (count + 1) } ,) { Err (_) => None , Ok (new_value) if new_value > usize :: MAX / 2 => { abort () ; } Ok (_) => Some (Receiver { channel : self . channel . clone () , listener : None , _pin : PhantomPinned , }) , } } } }
};
}

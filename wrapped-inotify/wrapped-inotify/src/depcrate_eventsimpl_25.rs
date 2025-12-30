// Generated macro for impl_25 (impl)
macro_rules! Depcrate_eventsimpl_25 {
() => {
// Module: crate::events
// Provides: {"impl_25"}
// Dependencies: {}
impl Display for EventMaskParseError { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: TooManyBitsSet (mask) => { writeln ! (f , "Error parsing event mask: too many event type bits set | {mask:?}") } Self :: QueueOverflow => writeln ! (f , "Error: the kernel's event queue overflowed") , } } }
};
}

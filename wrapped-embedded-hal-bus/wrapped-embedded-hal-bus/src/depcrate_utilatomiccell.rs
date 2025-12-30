// Generated macro for AtomicCell (struct)
macro_rules! Depcrate_utilAtomicCell {
() => {
// Module: crate::util
// Provides: {"AtomicCell"}
// Dependencies: {}
# [cfg (any (feature = "portable-atomic" , target_has_atomic = "8"))] # [doc = " Cell type used by [`spi::AtomicDevice`](crate::spi::AtomicDevice) and [`i2c::AtomicDevice`](crate::i2c::AtomicDevice)."] # [doc = ""] # [doc = " To use `AtomicDevice`, you must wrap the bus with this struct, and then"] # [doc = " construct multiple `AtomicDevice` instances with references to it."] pub struct AtomicCell < BUS > { pub (crate) bus : UnsafeCell < BUS > , pub (crate) busy : AtomicBool , }
};
}

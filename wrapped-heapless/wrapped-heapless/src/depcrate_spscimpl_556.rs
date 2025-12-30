// Generated macro for impl_556 (impl)
macro_rules! Depcrate_spscimpl_556 {
() => {
// Module: crate::spsc
// Provides: {"impl_556"}
// Dependencies: {}
impl < T , S > fmt :: Debug for QueueInner < T , S > where T : fmt :: Debug , S : Storage , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . iter ()) . finish () } }
};
}

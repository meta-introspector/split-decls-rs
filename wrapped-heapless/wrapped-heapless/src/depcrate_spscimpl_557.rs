// Generated macro for impl_557 (impl)
macro_rules! Depcrate_spscimpl_557 {
() => {
// Module: crate::spsc
// Provides: {"impl_557"}
// Dependencies: {}
impl < T , S > hash :: Hash for QueueInner < T , S > where T : hash :: Hash , S : Storage , { fn hash < H : hash :: Hasher > (& self , state : & mut H) { for t in self . iter () { hash :: Hash :: hash (t , state) ; } } }
};
}

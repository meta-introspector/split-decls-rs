// Generated macro for impl_648 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_648 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_648"}
// Dependencies: {}
impl < const N : usize > core :: fmt :: Debug for ArrayStr < N > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Debug :: fmt (self . as_str () , f) } }
};
}

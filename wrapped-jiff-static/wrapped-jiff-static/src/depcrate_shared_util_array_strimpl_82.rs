// Generated macro for impl_82 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_82 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_82"}
// Dependencies: {}
impl < const N : usize > core :: fmt :: Debug for ArrayStr < N > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Debug :: fmt (self . as_str () , f) } }
};
}

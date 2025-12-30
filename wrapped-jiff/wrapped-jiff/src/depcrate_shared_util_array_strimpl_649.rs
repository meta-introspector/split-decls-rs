// Generated macro for impl_649 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_649 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_649"}
// Dependencies: {}
impl < const N : usize > core :: fmt :: Display for ArrayStr < N > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { core :: fmt :: Display :: fmt (self . as_str () , f) } }
};
}

// Generated macro for impl_650 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_650 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_650"}
// Dependencies: {}
impl < const N : usize > core :: fmt :: Write for ArrayStr < N > { fn write_str (& mut self , s : & str) -> core :: fmt :: Result { if self . push_str (s) { Ok (()) } else { Err (core :: fmt :: Error) } } }
};
}

// Generated macro for impl_78 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_78 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_78"}
// Dependencies: {}
# [doc = " Easy construction of `ArrayStr` from `&'static str`."] # [doc = ""] # [doc = " We specifically limit to `&'static str` to approximate string literals."] # [doc = " This prevents most cases of accidentally creating a non-string literal"] # [doc = " that panics if the string is too big."] # [doc = ""] # [doc = " This impl primarily exists to make writing tests more convenient."] impl < const N : usize > From < & 'static str > for ArrayStr < N > { fn from (s : & 'static str) -> ArrayStr < N > { ArrayStr :: new (s) . unwrap () } }
};
}

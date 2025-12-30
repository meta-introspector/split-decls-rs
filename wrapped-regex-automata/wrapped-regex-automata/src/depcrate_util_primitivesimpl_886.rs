// Generated macro for impl_886 (impl)
macro_rules! Depcrate_util_primitivesimpl_886 {
() => {
// Module: crate::util::primitives
// Provides: {"impl_886"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for SmallIndex { fn arbitrary (gen : & mut quickcheck :: Gen) -> SmallIndex { use core :: cmp :: max ; let id = max (i32 :: MIN + 1 , i32 :: arbitrary (gen)) . abs () ; if id > SmallIndex :: MAX . as_i32 () { SmallIndex :: MAX } else { SmallIndex :: new (usize :: try_from (id) . unwrap ()) . unwrap () } } }
};
}

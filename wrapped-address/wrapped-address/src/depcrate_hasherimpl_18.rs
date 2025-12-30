// Generated macro for impl_18 (impl)
macro_rules! Depcrate_hasherimpl_18 {
() => {
// Module: crate::hasher
// Provides: {"impl_18"}
// Dependencies: {}
impl BuildHasher for AddressHasherBuilder { type Hasher = AddressHasher ; # [inline] fn build_hasher (& self) -> Self :: Hasher { AddressHasher { offset : self . offset , state : 0 , } } }
};
}

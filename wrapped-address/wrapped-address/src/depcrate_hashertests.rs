// Generated macro for tests (module)
macro_rules! Depcrate_hashertests {
() => {
// Module: crate::hasher
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: AddressHasherBuilder , crate :: Address , core :: hash :: { BuildHasher , Hasher } , } ; # [test] fn test_address_hasher_builder () { let key = Address :: new_unique () ; let builder = AddressHasherBuilder :: default () ; let mut hasher1 = builder . build_hasher () ; let mut hasher2 = builder . build_hasher () ; hasher1 . write (key . as_array ()) ; hasher2 . write (key . as_array ()) ; assert_eq ! (hasher1 . finish () , hasher2 . finish () , "Hashers made with same builder should be identical") ; let builder2 = AddressHasherBuilder :: default () ; for _ in 0 .. 64 { let mut hasher3 = builder2 . build_hasher () ; hasher3 . write (key . as_array ()) ; std :: dbg ! (hasher1 . finish ()) ; std :: dbg ! (hasher3 . finish ()) ; if hasher1 . finish () != hasher3 . finish () { return ; } } panic ! ("Hashers built with different builder should be different due to random offset") ; } # [test] fn test_address_hasher () { let key1 = Address :: new_unique () ; let key2 = Address :: new_unique () ; let builder = AddressHasherBuilder :: default () ; let mut hasher1 = builder . build_hasher () ; let mut hasher2 = builder . build_hasher () ; hasher1 . write (key1 . as_array ()) ; hasher2 . write (key2 . as_array ()) ; assert_ne ! (hasher1 . finish () , hasher2 . finish ()) ; } }
};
}

// Generated macro for issue_80303 (function)
macro_rules! Depcrate_collections_vec_deque_testsissue_80303 {
() => {
// Module: crate::collections::vec_deque::tests
// Provides: {"issue_80303"}
// Dependencies: {}
# [test] fn issue_80303 () { use core :: iter ; use core :: num :: Wrapping ; struct SimpleHasher (Wrapping < u64 >) ; impl Hasher for SimpleHasher { fn finish (& self) -> u64 { self . 0 . 0 } fn write (& mut self , bytes : & [u8]) { for & v in iter :: once (& 24) . chain (bytes) { self . 0 = Wrapping (31) * self . 0 + Wrapping (u64 :: from (v)) ; } } } fn hash_code (value : impl Hash) -> u64 { let mut hasher = SimpleHasher (Wrapping (1)) ; value . hash (& mut hasher) ; hasher . finish () } let vda : VecDeque < u8 > = (0 .. 10) . collect () ; let mut vdb = VecDeque :: with_capacity (10) ; vdb . extend (5 .. 10) ; (0 .. 5) . rev () . for_each (| elem | vdb . push_front (elem)) ; assert_ne ! (vda . as_slices () , vdb . as_slices ()) ; assert_eq ! (vda , vdb) ; assert_eq ! (hash_code (vda) , hash_code (vdb)) ; }
};
}

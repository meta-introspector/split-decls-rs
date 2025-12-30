// Generated macro for impl_14 (impl)
macro_rules! Depcrate_bufimpl_14 {
() => {
// Module: crate::buf
// Provides: {"impl_14"}
// Dependencies: {}
impl < const SIZE : usize > Ord for WriteBuffer < SIZE > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . as_str () . cmp (other . as_str ()) } }
};
}

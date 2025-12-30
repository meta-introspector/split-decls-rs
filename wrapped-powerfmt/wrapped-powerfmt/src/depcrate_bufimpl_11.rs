// Generated macro for impl_11 (impl)
macro_rules! Depcrate_bufimpl_11 {
() => {
// Module: crate::buf
// Provides: {"impl_11"}
// Dependencies: {}
impl < const LEFT_SIZE : usize , const RIGHT_SIZE : usize > PartialOrd < WriteBuffer < RIGHT_SIZE > > for WriteBuffer < LEFT_SIZE > { fn partial_cmp (& self , other : & WriteBuffer < RIGHT_SIZE >) -> Option < core :: cmp :: Ordering > { self . as_str () . partial_cmp (other . as_str ()) } }
};
}

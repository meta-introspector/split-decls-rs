// Generated macro for impl_12 (impl)
macro_rules! Depcrate_bufimpl_12 {
() => {
// Module: crate::buf
// Provides: {"impl_12"}
// Dependencies: {}
impl < const LEFT_SIZE : usize , const RIGHT_SIZE : usize > PartialEq < WriteBuffer < RIGHT_SIZE > > for WriteBuffer < LEFT_SIZE > { fn eq (& self , other : & WriteBuffer < RIGHT_SIZE >) -> bool { self . as_str () == other . as_str () } }
};
}

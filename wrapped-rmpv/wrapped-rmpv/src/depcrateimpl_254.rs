// Generated macro for impl_254 (impl)
macro_rules! Depcrateimpl_254 {
() => {
// Module: crate
// Provides: {"impl_254"}
// Dependencies: {}
impl < 'a > TryFrom < ValueRef < 'a > > for u64 { type Error = ValueRef < 'a > ; fn try_from (val : ValueRef < 'a >) -> Result < Self , Self :: Error > { match val { ValueRef :: Integer (n) => match n . as_u64 () { Some (i) => Ok (i) , None => Err (val) , } , v => Err (v) , } } }
};
}

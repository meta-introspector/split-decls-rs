// Generated macro for impl_227 (impl)
macro_rules! Depcrate_param_valueimpl_227 {
() => {
// Module: crate::param_value
// Provides: {"impl_227"}
// Dependencies: {}
impl < T : Type < T > > ParamValue < T > { pub fn abi (& self) -> T :: Abi { unsafe { match self { Self :: Owned (item) => transmute_copy (item) , Self :: Borrowed (borrowed) => transmute_copy (borrowed) , } } } pub fn borrow (& self) -> Ref < '_ , T > { unsafe { transmute_copy (& self . abi ()) } } }
};
}

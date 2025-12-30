// Generated macro for impl_140 (impl)
macro_rules! Depcrate_astimpl_140 {
() => {
// Module: crate::ast
// Provides: {"impl_140"}
// Dependencies: {}
impl < 'a , N , M > From < & 'a Type < N , M > > for BorrowedType < 'a > where N : AsRef < str > , M : AsRef < [TypeModifier] > , { fn from (value : & 'a Type < N , M >) -> Self { Self { name : value . name . as_ref () , modifiers : value . modifiers . as_ref () , } } }
};
}

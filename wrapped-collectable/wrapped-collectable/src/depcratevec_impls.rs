// Generated macro for vec_impls (module)
macro_rules! Depcratevec_impls {
() => {
// Module: crate
// Provides: {"vec_impls"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod vec_impls { use super :: { Length , Truncate , TryExtend } ; use alloc :: vec :: Vec ; use core :: convert :: Infallible ; impl < T > Length for Vec < T > { fn len (& self) -> usize { Vec :: len (self) } } impl < T > Truncate for Vec < T > { fn truncate (& mut self , len : usize) { Vec :: truncate (self , len) ; } } impl < A > TryExtend < A > for Vec < A > { type Error = Infallible ; fn try_extend < T : IntoIterator < Item = A > > (& mut self , iter : T) -> Result < () , Infallible > { Vec :: extend (self , iter) ; Ok (()) } } }
};
}

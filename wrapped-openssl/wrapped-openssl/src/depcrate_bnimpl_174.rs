// Generated macro for impl_174 (impl)
macro_rules! Depcrate_bnimpl_174 {
() => {
// Module: crate::bn
// Provides: {"impl_174"}
// Dependencies: {}
impl PartialOrd < BigNum > for BigNumRef { fn partial_cmp (& self , oth : & BigNum) -> Option < Ordering > { Some (self . cmp (oth . deref ())) } }
};
}

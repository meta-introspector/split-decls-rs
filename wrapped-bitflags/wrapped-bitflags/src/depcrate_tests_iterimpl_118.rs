// Generated macro for impl_118 (impl)
macro_rules! Depcrate_tests_iterimpl_118 {
() => {
// Module: crate::tests::iter
// Provides: {"impl_118"}
// Dependencies: {}
impl < B : 'static > IterNames < B > { # [doc (hidden)] pub const fn __private_const_new (flags : & 'static [Flag < B >] , source : B , remaining : B) -> Self { IterNames { flags , idx : 0 , remaining , source , } } # [doc = " Get a flags value of any remaining bits that haven't been yielded yet."] # [doc = ""] # [doc = " Once the iterator has finished, this method can be used to"] # [doc = " check whether or not there are any bits that didn't correspond"] # [doc = " to a contained, defined, named flag remaining."] pub fn remaining (& self) -> & B { & self . remaining } }
};
}

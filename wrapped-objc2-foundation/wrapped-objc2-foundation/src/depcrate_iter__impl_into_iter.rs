// Generated macro for __impl_into_iter (macro)
macro_rules! Depcrate_iter__impl_into_iter {
() => {
// Module: crate::iter
// Provides: {"__impl_into_iter"}
// Dependencies: {}
# [doc (hidden)] macro_rules ! __impl_into_iter { () => { } ; ($ (# [$ m : meta]) * impl <$ param : ident : Message > IntoIterator for &$ ty : ident <$ param2 : ident > { type IntoIter = $ iter : ident <'_ , $ param3 : ident >; } $ ($ rest : tt) *) => { $ (# [$ m]) * impl <'a , $ param : Message > IntoIterator for &'a $ ty <$ param2 > { type Item = Retained <$ param3 >; type IntoIter = $ iter <'a , $ param3 >; # [inline] fn into_iter (self) -> Self :: IntoIter { $ iter ($ crate :: iter :: Iter :: new (& self)) } } __impl_into_iter ! { $ ($ rest) * } } ; ($ (# [$ m : meta]) * impl <$ param : ident : Message > IntoIterator for Retained <$ ty : ident <$ param2 : ident >> { # [uses ($ new_fn : ident)] type IntoIter = $ into_iter : ident <$ param3 : ident >; } $ ($ rest : tt) *) => { $ (# [$ m]) * impl <$ param : Message > objc2 :: rc :: RetainedIntoIterator for $ ty <$ param2 > { type Item = Retained <$ param3 >; type IntoIter = $ into_iter <$ param3 >; # [inline] fn retained_into_iter (this : Retained < Self >) -> Self :: IntoIter { $ into_iter ($ crate :: iter :: IntoIter ::$ new_fn (this)) } } __impl_into_iter ! { $ ($ rest) * } } ; }
};
}

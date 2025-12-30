// Generated macro for impl_try_from_ref (macro)
macro_rules! Depcrateimpl_try_from_ref {
() => {
// Module: crate
// Provides: {"impl_try_from_ref"}
// Dependencies: {}
macro_rules ! impl_try_from_ref { ($ t : ty , $ p : ident) => { impl <'a > TryFrom < ValueRef <'a >> for $ t { type Error = ValueRef <'a >; fn try_from (val : ValueRef <'a >) -> Result <$ t , Self :: Error > { match val { ValueRef ::$ p (v) => Ok (v) , v => Err (v) , } } } } ; }
};
}

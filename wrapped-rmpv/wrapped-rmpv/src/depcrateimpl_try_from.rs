// Generated macro for impl_try_from (macro)
macro_rules! Depcrateimpl_try_from {
() => {
// Module: crate
// Provides: {"impl_try_from"}
// Dependencies: {}
macro_rules ! impl_try_from { ($ t : ty , $ p : ident) => { impl TryFrom < Value > for $ t { type Error = Value ; fn try_from (val : Value) -> Result <$ t , Self :: Error > { match val { Value ::$ p (v) => Ok (v) , v => Err (v) , } } } } ; }
};
}

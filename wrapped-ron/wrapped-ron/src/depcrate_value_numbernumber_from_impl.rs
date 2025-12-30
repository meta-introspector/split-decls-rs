// Generated macro for number_from_impl (macro)
macro_rules! Depcrate_value_numbernumber_from_impl {
() => {
// Module: crate::value::number
// Provides: {"number_from_impl"}
// Dependencies: {}
macro_rules ! number_from_impl { (Number ::$ variant : ident ($ wrap : ident ($ ty : ty))) => { impl From <$ ty > for Number { fn from (v : $ ty) -> Number { Number ::$ variant ($ wrap (v)) } } } ; (Number ::$ variant : ident ($ ty : ty)) => { impl From <$ ty > for Number { fn from (v : $ ty) -> Number { Number ::$ variant (v) } } } ; }
};
}

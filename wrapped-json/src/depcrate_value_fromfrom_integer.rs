// Generated macro for from_integer (macro)
macro_rules! Depcrate_value_fromfrom_integer {
() => {
// Module: crate::value::from
// Provides: {"from_integer"}
// Dependencies: {}
macro_rules ! from_integer { ($ ($ ty : ident) *) => { $ (impl From <$ ty > for Value { fn from (n : $ ty) -> Self { Value :: Number (n . into ()) } }) * } ; }
};
}

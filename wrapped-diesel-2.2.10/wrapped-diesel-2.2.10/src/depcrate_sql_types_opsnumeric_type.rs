// Generated macro for numeric_type (macro)
macro_rules! Depcrate_sql_types_opsnumeric_type {
() => {
// Module: crate::sql_types::ops
// Provides: {"numeric_type"}
// Dependencies: {}
macro_rules ! numeric_type { ($ ($ tpe : ident) ,*) => { $ (impl Add for $ tpe { type Rhs = $ tpe ; type Output = $ tpe ; } impl Sub for $ tpe { type Rhs = $ tpe ; type Output = $ tpe ; } impl Mul for $ tpe { type Rhs = $ tpe ; type Output = $ tpe ; } impl Div for $ tpe { type Rhs = $ tpe ; type Output = $ tpe ; }) * } }
};
}

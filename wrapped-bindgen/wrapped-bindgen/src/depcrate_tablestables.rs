// Generated macro for tables (macro)
macro_rules! Depcrate_tablestables {
() => {
// Module: crate::tables
// Provides: {"tables"}
// Dependencies: {}
macro_rules ! tables { ($ (($ name : ident , $ table : literal)) +) => { $ (# [derive (Copy , Clone , Hash , PartialEq , Eq , Ord , PartialOrd)] pub struct $ name (pub Row) ; impl AsRow for $ name { const TABLE : usize = $ table ; fn to_row (& self) -> Row { self . 0 } fn from_row (row : Row) -> Self { $ name (row) } }) * } ; }
};
}

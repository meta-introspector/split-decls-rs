// Generated macro for tables (macro)
macro_rules! Depcrate_reader_tablestables {
() => {
// Module: crate::reader::tables
// Provides: {"tables"}
// Dependencies: {}
macro_rules ! tables { ($ (($ name : ident , $ table : literal)) +) => { $ (# [derive (Copy , Clone , Hash , PartialEq , Eq , Ord , PartialOrd)] pub struct $ name <'a > (pub (crate) Row <'a >) ; impl <'a > AsRow <'a > for $ name <'a > { const TABLE : usize = $ table ; fn to_row (& self) -> Row <'a > { self . 0 } fn from_row (row : Row <'a >) -> Self { $ name (row) } }) * } ; }
};
}

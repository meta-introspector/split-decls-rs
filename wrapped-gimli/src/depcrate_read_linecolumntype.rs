// Generated macro for ColumnType (enum)
macro_rules! Depcrate_read_lineColumnType {
() => {
// Module: crate::read::line
// Provides: {"ColumnType"}
// Dependencies: {}
# [doc = " The type of column that a row is referring to."] # [derive (Copy , Clone , Debug , PartialEq , Eq , PartialOrd , Ord)] pub enum ColumnType { # [doc = " The `LeftEdge` means that the statement begins at the start of the new"] # [doc = " line."] LeftEdge , # [doc = " A column number, whose range begins at 1."] Column (NonZeroU64) , }
};
}

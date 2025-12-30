// Generated macro for impl_3696 (impl)
macro_rules! Depcrate_sqlite_connection_functionsimpl_3696 {
() => {
// Module: crate::sqlite::connection::functions
// Provides: {"impl_3696"}
// Dependencies: {}
impl < 'a > Row < 'a , Sqlite > for FunctionRow < 'a > { type Field < 'f > = FunctionArgument < 'f > where 'a : 'f , Self : 'f ; type InnerPartialRow = Self ; fn field_count (& self) -> usize { self . field_count } fn get < 'b , I > (& 'b self , idx : I) -> Option < Self :: Field < 'b > > where 'a : 'b , Self : crate :: row :: RowIndex < I > , { let col_idx = self . idx (idx) ? ; Some (FunctionArgument { args : self . args . borrow () , col_idx , }) } fn partial_row (& self , range : std :: ops :: Range < usize >) -> PartialRow < '_ , Self :: InnerPartialRow > { PartialRow :: new (self , range) } }
};
}

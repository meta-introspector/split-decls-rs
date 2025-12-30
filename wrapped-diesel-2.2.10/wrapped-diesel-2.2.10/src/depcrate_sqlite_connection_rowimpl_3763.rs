// Generated macro for impl_3763 (impl)
macro_rules! Depcrate_sqlite_connection_rowimpl_3763 {
() => {
// Module: crate::sqlite::connection::row
// Provides: {"impl_3763"}
// Dependencies: {}
impl < 'stmt > Row < 'stmt , Sqlite > for SqliteRow < 'stmt , '_ > { type Field < 'field > = SqliteField < 'field , 'field > where 'stmt : 'field , Self : 'field ; type InnerPartialRow = Self ; fn field_count (& self) -> usize { self . field_count } fn get < 'field , I > (& 'field self , idx : I) -> Option < Self :: Field < 'field > > where 'stmt : 'field , Self : RowIndex < I > , { let idx = self . idx (idx) ? ; Some (SqliteField { row : self . inner . borrow () , col_idx : idx , }) } fn partial_row (& self , range : std :: ops :: Range < usize >) -> PartialRow < '_ , Self :: InnerPartialRow > { PartialRow :: new (self , range) } }
};
}
